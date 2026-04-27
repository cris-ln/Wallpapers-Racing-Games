// ─────────────────────────────────────────────────────────────────
// CLIENT.TS — WRG Wallpaper Gaming Marketplace
// Script para interactuar con el programa desde Solana Playground.
// Ejecuta cada sección una por una cambiando la variable ACCION.
// ─────────────────────────────────────────────────────────────────

// ── Cambia este valor para elegir qué instrucción ejecutar ────────
// Opciones: "createCreator" | "listWallpaper" | "addToWishlist" | "removeFromWishlist" | "closeListing" | "fetchAll"
const ACCION = "fetchAll";

// ── Dirección del wallpaper (se obtiene después de listWallpaper) ─
const WALLPAPER_ADDRESS = "8TgygibtP1e1Hh3bRf9FrqbnhYeRFZRzqhFBszj4x6Yj";

// ─────────────────────────────────────────────────────────────────
// Derivar PDAs
// ─────────────────────────────────────────────────────────────────
const [creadorPDA] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("creator"), pg.wallet.publicKey.toBuffer()],
  pg.PROGRAM_ID
);

const [wishlistPDA] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("wishlist"), pg.wallet.publicKey.toBuffer()],
  pg.PROGRAM_ID
);

console.log("── Direcciones ──────────────────────────────");
console.log("Tu wallet:  ", pg.wallet.publicKey.toBase58());
console.log("PDA creador:", creadorPDA.toBase58());
console.log("PDA wishlist:", wishlistPDA.toBase58());
console.log("─────────────────────────────────────────────");

// ─────────────────────────────────────────────────────────────────
// INSTRUCCIÓN 1: createCreator
// Crea el perfil on-chain del artista.
// Solo se puede ejecutar una vez por wallet.
// ─────────────────────────────────────────────────────────────────
if (ACCION === "createCreator") {
  const tx = await pg.program.methods
    .createCreator("Suprenmus")
    .accounts({
      artista: pg.wallet.publicKey,
      creador: creadorPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ createCreator exitoso!");
  console.log("Tx:", tx);
}

// ─────────────────────────────────────────────────────────────────
// INSTRUCCIÓN 2: listWallpaper
// Lista un nuevo wallpaper en el marketplace.
// Requiere tener perfil de creador (createCreator).
// ─────────────────────────────────────────────────────────────────
if (ACCION === "listWallpaper") {
  // Obtenemos el índice actual del creador para derivar la PDA correcta
  const creadorCuenta = await pg.program.account.creador.fetch(creadorPDA);
  const indice = creadorCuenta.totalWallpapers;
  const indiceBuffer = indice.toArrayLike(Buffer, "le", 8);

  const [wallpaperPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("wallpaper"), pg.wallet.publicKey.toBuffer(), indiceBuffer],
    pg.PROGRAM_ID
  );

  console.log("PDA wallpaper:", wallpaperPDA.toBase58());

  const tx = await pg.program.methods
    .listWallpaper(
      "1995 Mazda MX-5",       // título
      "Forza Horizon 6",        // juego
      new BN(50000000),        // precio: 0.05 SOL en lamports
      new BN(100),             // ediciones totales
      "https://arweave.net/test" // URI de la imagen
    )
    .accounts({
      artista: pg.wallet.publicKey,
      creador: creadorPDA,
      wallpaper: wallpaperPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ listWallpaper exitoso!");
  console.log("Tx:", tx);
  console.log("⚠️  Guarda esta dirección del wallpaper:", wallpaperPDA.toBase58());
}

// ─────────────────────────────────────────────────────────────────
// INSTRUCCIÓN 3: addToWishlist
// Añade un wallpaper a la wishlist del usuario.
// Usa la dirección de WALLPAPER_ADDRESS arriba.
// ─────────────────────────────────────────────────────────────────
if (ACCION === "addToWishlist") {
  const wallpaperPDA = new web3.PublicKey(WALLPAPER_ADDRESS);

  const tx = await pg.program.methods
    .addToWishlist()
    .accounts({
      usuario: pg.wallet.publicKey,
      wishlist: wishlistPDA,
      wallpaper: wallpaperPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ addToWishlist exitoso!");
  console.log("Tx:", tx);
}

// ─────────────────────────────────────────────────────────────────
// INSTRUCCIÓN 4: removeFromWishlist
// Elimina un wallpaper de la wishlist del usuario.
// ─────────────────────────────────────────────────────────────────
if (ACCION === "removeFromWishlist") {
  const wallpaperPDA = new web3.PublicKey(WALLPAPER_ADDRESS);

  const tx = await pg.program.methods
    .removeFromWishlist()
    .accounts({
      usuario: pg.wallet.publicKey,
      wishlist: wishlistPDA,
      wallpaper: wallpaperPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ removeFromWishlist exitoso!");
  console.log("Tx:", tx);
}

// ─────────────────────────────────────────────────────────────────
// INSTRUCCIÓN 5: closeListing
// Cierra el listado del wallpaper y devuelve el rent al artista.
// ─────────────────────────────────────────────────────────────────
if (ACCION === "closeListing") {
  const wallpaperPDA = new web3.PublicKey(WALLPAPER_ADDRESS);

  const tx = await pg.program.methods
    .closeListing()
    .accounts({
      artista: pg.wallet.publicKey,
      wallpaper: wallpaperPDA,
      creador: creadorPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("✅ closeListing exitoso!");
  console.log("Tx:", tx);
}

// ─────────────────────────────────────────────────────────────────
// FETCH ALL — Lee todas las cuentas existentes
// ─────────────────────────────────────────────────────────────────
if (ACCION === "fetchAll") {
  console.log("\n── Creadores ────────────────────────────────");
  const creadores = await pg.program.account.creador.all();
  creadores.forEach((c) => {
    console.log("Nombre:", c.account.nombre);
    console.log("Owner:", c.account.owner.toBase58());
    console.log("Total wallpapers:", c.account.totalWallpapers.toString());
    console.log("PDA:", c.publicKey.toBase58());
    console.log("────────────────────────────────────────────");
  });

  console.log("\n── Wallpapers ───────────────────────────────");
  const wallpapers = await pg.program.account.wallpaper.all();
  wallpapers.forEach((w) => {
    console.log("Título:", w.account.titulo);
    console.log("Juego:", w.account.juego);
    console.log("Precio:", w.account.precio.toString(), "lamports");
    console.log("Ediciones:", w.account.vendidas?.toString() ?? "0", "/", w.account.total.toString());
    console.log("En venta:", w.account.enVenta);
    console.log("PDA:", w.publicKey.toBase58());
    console.log("────────────────────────────────────────────");
  });

  console.log("\n── Wishlists ────────────────────────────────");
  const wishlists = await pg.program.account.wishlist.all();
  wishlists.forEach((wl) => {
    console.log("Owner:", wl.account.owner.toBase58());
    console.log("Wallpapers guardados:", wl.account.wallpapers.length);
    wl.account.wallpapers.forEach((w, i) => {
      console.log(`  ${i + 1}.`, w.toBase58());
    });
    console.log("────────────────────────────────────────────");
  });
}