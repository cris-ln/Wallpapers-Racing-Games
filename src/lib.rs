use anchor_lang::prelude::*; // Importa todo lo necesario de Anchor (macros, traits, tipos)

// El Program ID es la dirección única de tu programa en la blockchain de Solana.
// Este valor lo genera Anchor automáticamente y debe coincidir con el keypair del programa.
declare_id!("8kZC36vAknnDvuGcU6fSmw5GGHgNUpGv4Nf2pvTuS2Zg");

// La macro #[program] marca este módulo como el punto de entrada del programa.
// Todas las funciones públicas dentro de este módulo son las "instrucciones" que
// los usuarios pueden llamar desde el exterior (frontend, tests, CLI, etc).
#[program]
pub mod wrg {
    use super::*; // Importa los structs, enums y contextos definidos más abajo

    // ─────────────────────────────────────────────────────────────────
    // INSTRUCCIÓN 1: create_creator
    // Crea el perfil on-chain de un artista.
    // Cada wallet solo puede tener un perfil (porque la PDA es única por wallet).
    // ─────────────────────────────────────────────────────────────────
    pub fn create_creator(ctx: Context<NuevoCreador>, nombre: String) -> Result<()> {

        // Validamos que el nombre no supere los 50 caracteres.
        // Si lo supera, lanzamos nuestro error personalizado NombreLargo.
        require!(nombre.as_bytes().len() <= 50, Errores::NombreLargo);

        // Obtenemos una referencia mutable a la cuenta del creador para poder escribir en ella.
        let creador = &mut ctx.accounts.creador;

        // Guardamos los datos del artista en la cuenta on-chain.
        creador.owner            = ctx.accounts.artista.key(); // La wallet que firma la transacción
        creador.nombre           = nombre.clone();             // El nombre del artista
        creador.total_wallpapers = 0;                          // Empieza sin wallpapers
        creador.bump             = ctx.bumps.creador;          // El bump de la PDA (lo guarda Anchor)

        msg!("Creador registrado: {}", nombre); // Log visible en el explorador de Solana
        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────
    // INSTRUCCIÓN 2: list_wallpaper
    // Lista un nuevo wallpaper en el marketplace.
    // Solo puede llamarla alguien que ya tenga perfil de creador.
    // ─────────────────────────────────────────────────────────────────
    pub fn list_wallpaper(
        ctx: Context<NuevoWallpaper>,
        titulo: String,
        juego: String,
        precio: u64,   // En lamports (1 SOL = 1,000,000,000 lamports)
        total: u64,    // Número de ediciones disponibles
        uri: String,   // URL de la imagen (Arweave, IPFS, etc.)
    ) -> Result<()> {

        // Validaciones básicas antes de crear el wallpaper
        require!(total > 0, Errores::EdicionesInvalidas); // Debe tener al menos 1 edición
        require!(precio > 0, Errores::PrecioInvalido);    // El precio no puede ser 0

        let creador  = &mut ctx.accounts.creador;
        let wallpaper = &mut ctx.accounts.wallpaper;

        // Llenamos la cuenta del wallpaper con los datos recibidos
        wallpaper.creador  = ctx.accounts.artista.key(); // Quién lo creó
        wallpaper.titulo   = titulo.clone();              // Nombre del wallpaper
        wallpaper.juego    = juego;                       // Juego al que pertenece
        wallpaper.precio   = precio;                      // Precio en lamports
        wallpaper.total    = total;                       // Ediciones totales
        wallpaper.en_venta = true;                        // Empieza disponible para listar
        wallpaper.uri      = uri;                         // URL de la imagen
        wallpaper.bump     = ctx.bumps.wallpaper;         // Bump de la PDA

        // Guardamos el índice actual del creador como parte del wallpaper.
        // Esto es clave: el índice se usa como seed para derivar la PDA del wallpaper,
        // así dos wallpapers con el mismo título no generan la misma dirección.
        wallpaper.indice = creador.total_wallpapers;

        // Incrementamos el contador de wallpapers del creador.
        // checked_add(1) es una suma segura que evita overflow (desbordamiento numérico).
        creador.total_wallpapers = creador.total_wallpapers.checked_add(1).unwrap();

        msg!("Wallpaper listado: {}", titulo);
        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────
    // INSTRUCCIÓN 3: add_to_wishlist
    // Añade un wallpaper a la lista de deseados del usuario.
    // Si el usuario no tiene wishlist aún, se crea automáticamente (init_if_needed).
    // ─────────────────────────────────────────────────────────────────
    pub fn add_to_wishlist(ctx: Context<ModificarWishlist>) -> Result<()> {
        let wishlist = &mut ctx.accounts.wishlist;

        // Si bump == 0, significa que la cuenta acaba de ser creada por primera vez.
        // La inicializamos con el owner y el bump.
        if wishlist.bump == 0 {
            wishlist.owner = ctx.accounts.usuario.key();
            wishlist.bump  = ctx.bumps.wishlist;
        }

        // Verificamos que la wishlist no esté llena (máximo 20 wallpapers).
        // Esto es importante porque el espacio de la cuenta está fijo en el momento de crearla.
        require!(wishlist.wallpapers.len() < 20, Errores::WishlistLlena);

        // Obtenemos la dirección (pubkey) del wallpaper que queremos guardar
        let wallpaper_key = ctx.accounts.wallpaper.key();

        // Verificamos que ese wallpaper no esté ya en la wishlist
        require!(
            !wishlist.wallpapers.contains(&wallpaper_key),
            Errores::YaEnWishlist
        );

        // Añadimos la dirección del wallpaper al vector de la wishlist
        wishlist.wallpapers.push(wallpaper_key);

        msg!("Añadido a wishlist. Total: {}", wishlist.wallpapers.len());
        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────
    // INSTRUCCIÓN 4: remove_from_wishlist
    // Elimina un wallpaper de la lista de deseados del usuario.
    // ─────────────────────────────────────────────────────────────────
    pub fn remove_from_wishlist(ctx: Context<ModificarWishlist>) -> Result<()> {
        let wishlist     = &mut ctx.accounts.wishlist;
        let wallpaper_key = ctx.accounts.wallpaper.key();

        // Buscamos la posición del wallpaper en el vector.
        // Si no existe, lanzamos el error NoEnWishlist.
        let pos = wishlist
            .wallpapers
            .iter()
            .position(|&x| x == wallpaper_key)
            .ok_or(Errores::NoEnWishlist)?;

        // Eliminamos el wallpaper de esa posición
        wishlist.wallpapers.remove(pos);

        msg!("Eliminado de wishlist. Total: {}", wishlist.wallpapers.len());
        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────
    // INSTRUCCIÓN 5: close_listing
    // Cierra el listado de un wallpaper y elimina su cuenta on-chain.
    // Al cerrar la cuenta, el rent (SOL bloqueado) se devuelve al artista.
    // Solo el creador del wallpaper puede cerrarlo, y solo si está en venta.
    // ─────────────────────────────────────────────────────────────────
    pub fn close_listing(ctx: Context<CerrarListado>) -> Result<()> {

        // Bajamos el contador de wallpapers del creador.
        // checked_sub(1) es una resta segura que evita underflow.
        ctx.accounts.creador.total_wallpapers = ctx
            .accounts
            .creador
            .total_wallpapers
            .checked_sub(1)
            .unwrap();

        // La cuenta del wallpaper se cierra automáticamente gracias a
        // la constraint `close = artista` definida en el contexto CerrarListado.
        msg!("Listado cerrado: {}", ctx.accounts.wallpaper.titulo);
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ERRORES PERSONALIZADOS
// Anchor permite definir errores con mensajes claros.
// Cuando una instrucción falla, el cliente recibe este mensaje.
// ─────────────────────────────────────────────────────────────────────────────
#[error_code]
pub enum Errores {
    #[msg("No autorizado")]
    NoAutorizado,
    #[msg("Nombre demasiado largo (máximo 50 caracteres)")]
    NombreLargo,
    #[msg("El número de ediciones debe ser mayor a 0")]
    EdicionesInvalidas,
    #[msg("El precio debe ser mayor a 0")]
    PrecioInvalido,
    #[msg("Ya está en la wishlist")]
    YaEnWishlist,
    #[msg("No está en la wishlist")]
    NoEnWishlist,
    #[msg("Wishlist llena (máximo 20 wallpapers)")]
    WishlistLlena,
    #[msg("Wallpaper no está en venta")]
    NoEnVenta,
}

// ─────────────────────────────────────────────────────────────────────────────
// CUENTAS (structs que viven on-chain)
// Cada struct representa una cuenta que Anchor serializa y guarda en la blockchain.
// #[derive(InitSpace)] calcula automáticamente el espacio necesario.
// #[max_len(N)] indica el tamaño máximo de Strings y Vecs.
// ─────────────────────────────────────────────────────────────────────────────

// Perfil del artista — una por wallet
#[account]
#[derive(InitSpace)]
pub struct Creador {
    pub owner: Pubkey,          // Wallet del artista (32 bytes)
    #[max_len(50)]
    pub nombre: String,         // Nombre o alias (hasta 50 caracteres)
    pub total_wallpapers: u64,  // Cuántos wallpapers tiene activos (se usa como seed)
    pub bump: u8,               // Bump de la PDA — lo guarda Anchor para re-derivarla
}

// Cada wallpaper listado en el marketplace
#[account]
#[derive(InitSpace)]
pub struct Wallpaper {
    pub creador: Pubkey,   // Quién lo creó
    #[max_len(60)]
    pub titulo: String,    // Nombre del wallpaper
    #[max_len(30)]
    pub juego: String,     // Videojuego al que pertenece
    pub precio: u64,       // Precio en lamports
    pub total: u64,        // Ediciones totales disponibles
    pub en_venta: bool,    // Si está activo en el marketplace
    #[max_len(100)]
    pub uri: String,       // URL de la imagen del wallpaper
    pub indice: u64,       // Índice único dentro del perfil del artista (seed de la PDA)
    pub bump: u8,          // Bump de la PDA
}

// Lista de wallpapers deseados por un usuario — una por wallet
#[account]
#[derive(InitSpace)]
pub struct Wishlist {
    pub owner: Pubkey,          // Wallet del usuario
    #[max_len(20)]
    pub wallpapers: Vec<Pubkey>, // Lista de PDAs de wallpapers guardados (máx 20)
    pub bump: u8,               // Bump de la PDA
}

// ─────────────────────────────────────────────────────────────────────────────
// CONTEXTOS
// Cada contexto define qué cuentas recibe una instrucción y qué validaciones
// debe cumplir cada una antes de ejecutar la lógica.
// ─────────────────────────────────────────────────────────────────────────────

// Contexto para: create_creator
#[derive(Accounts)]
pub struct NuevoCreador<'info> {
    #[account(mut)] // mut porque paga el rent de la nueva cuenta
    pub artista: Signer<'info>, // Debe firmar la transacción

    #[account(
        init,                               // Crea la cuenta si no existe
        payer = artista,                    // El artista paga el rent
        space = 8 + Creador::INIT_SPACE,    // 8 bytes de discriminador + espacio del struct
        seeds = [b"creator", artista.key().as_ref()], // PDA única por wallet
        bump                                // Anchor encuentra el bump automáticamente
    )]
    pub creador: Account<'info, Creador>,

    pub system_program: Program<'info, System>, // Requerido para crear cuentas
}

// Contexto para: list_wallpaper
#[derive(Accounts)]
pub struct NuevoWallpaper<'info> {
    #[account(mut)]
    pub artista: Signer<'info>,

    #[account(
        mut,                                         // mut porque actualizamos total_wallpapers
        seeds = [b"creator", artista.key().as_ref()],
        bump = creador.bump,                         // Usamos el bump guardado para verificar la PDA
        constraint = creador.owner == artista.key() @ Errores::NoAutorizado // Solo el dueño puede listar
    )]
    pub creador: Account<'info, Creador>,

    #[account(
        init,
        payer = artista,
        space = 8 + Wallpaper::INIT_SPACE,
        // La seed usa el índice actual del creador — así cada wallpaper tiene PDA única
        // aunque dos wallpapers tengan el mismo título
        seeds = [b"wallpaper", artista.key().as_ref(), &creador.total_wallpapers.to_le_bytes()],
        bump
    )]
    pub wallpaper: Account<'info, Wallpaper>,

    pub system_program: Program<'info, System>,
}

// Contexto para: add_to_wishlist y remove_from_wishlist
// Ambas instrucciones usan el mismo contexto porque necesitan las mismas cuentas
#[derive(Accounts)]
pub struct ModificarWishlist<'info> {
    #[account(mut)]
    pub usuario: Signer<'info>,

    #[account(
        init_if_needed,                              // Crea la cuenta si no existe, o la reutiliza
        payer = usuario,
        space = 8 + Wishlist::INIT_SPACE,
        seeds = [b"wishlist", usuario.key().as_ref()],
        bump
    )]
    pub wishlist: Account<'info, Wishlist>,

    /// CHECK: Solo necesitamos la pubkey del wallpaper para guardarla en la lista.
    /// No leemos ni escribimos datos de esta cuenta, por eso es UncheckedAccount.
    pub wallpaper: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

// Contexto para: close_listing
#[derive(Accounts)]
pub struct CerrarListado<'info> {
    #[account(mut)]
    pub artista: Signer<'info>,

    #[account(
        mut,
        close = artista, // Al terminar la instrucción, cierra esta cuenta y manda el rent al artista
        seeds = [b"wallpaper", artista.key().as_ref(), &wallpaper.indice.to_le_bytes()],
        bump = wallpaper.bump,
        constraint = wallpaper.creador == artista.key() @ Errores::NoAutorizado, // Solo el creador puede cerrar
        constraint = wallpaper.en_venta == true @ Errores::NoEnVenta             // Solo si está en venta
    )]
    pub wallpaper: Account<'info, Wallpaper>,

    #[account(
        mut,
        seeds = [b"creator", artista.key().as_ref()],
        bump = creador.bump,
        constraint = creador.owner == artista.key() @ Errores::NoAutorizado
    )]
    pub creador: Account<'info, Creador>,

    pub system_program: Program<'info, System>,
}