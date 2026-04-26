# Wallpapers Racing Games

Este proyecto implementa un programa inteligente en Rust para la blockchain de Solana, diseñado para gestionar un marketplace de wallpapers de videojuegos como ediciones limitadas. El programa permite crear un perfil de artista, listar wallpapers, gestionar una wishlist personal y cerrar listados.

## Descripción

El módulo `wrg` es un programa inteligente escrito en Rust con el framework Anchor para la blockchain de Solana. Permite gestionar un marketplace descentralizado donde los artistas pueden listar wallpapers de videojuegos como ediciones limitadas, y los usuarios pueden guardarlos en una wishlist personal. El programa incluye funciones para crear perfiles de creador, listar wallpapers con precio y ediciones limitadas, añadir y eliminar wallpapers de una wishlist, y cerrar listados recuperando el rent. El programa está diseñado para ser simple pero funcional, con validaciones básicas como verificar que el precio y las ediciones sean mayores a 0, y que el nombre del artista no supere los 50 caracteres.

## Dependencias

El programa utiliza los siguientes módulos y frameworks:

- **anchor-lang**: Framework principal para el desarrollo de programas en Solana. Proporciona macros, traits y tipos esenciales.
- **`#[derive(InitSpace)]`**: Calcula automáticamente el espacio necesario para almacenar cada cuenta en la blockchain.
- **`#[max_len(N)]`**: Define el tamaño máximo de Strings y Vecs dentro de las cuentas.
- **`init_if_needed`**: Feature de Anchor que permite crear una cuenta solo si no existe previamente.

## Estructura del Proyecto

```
src/
└── lib.rs          # Punto de entrada del programa. Contiene instrucciones, cuentas, contextos y errores.
client/
└── client.ts       # Script para interactuar con el programa desde Solana Playground.
tests/
└── anchor.test.ts  # Tests automatizados del programa.
```

## Cuentas On-Chain

El programa define los siguientes structs que se almacenan en la blockchain:

**Creador**: Perfil del artista. Se deriva como PDA usando `["creator", wallet_del_artista]`.

| Campo | Tipo | Descripción |
|---|---|---|
| `owner` | Pubkey | Wallet del artista |
| `nombre` | String | Nombre o alias del artista (máx 50 caracteres) |
| `total_wallpapers` | u64 | Wallpapers activos en el marketplace |
| `bump` | u8 | Bump de la PDA |

**Wallpaper**: Cada wallpaper listado en el marketplace. Se deriva como PDA usando `["wallpaper", wallet_del_artista, indice]`.

| Campo | Tipo | Descripción |
|---|---|---|
| `creador` | Pubkey | Wallet del artista que lo creó |
| `titulo` | String | Nombre del wallpaper (máx 60 caracteres) |
| `juego` | String | Videojuego al que pertenece (máx 30 caracteres) |
| `precio` | u64 | Precio en lamports (1 SOL = 1,000,000,000 lamports) |
| `total` | u64 | Ediciones totales disponibles |
| `en_venta` | bool | Si está activo en el marketplace |
| `uri` | String | URL de la imagen del wallpaper (máx 100 caracteres) |
| `indice` | u64 | Índice único dentro del perfil del artista (seed de la PDA) |
| `bump` | u8 | Bump de la PDA |

**Wishlist**: Lista de wallpapers guardados por un usuario. Se deriva como PDA usando `["wishlist", wallet_del_usuario]`.

| Campo | Tipo | Descripción |
|---|---|---|
| `owner` | Pubkey | Wallet del usuario |
| `wallpapers` | Vec\<Pubkey\> | Lista de PDAs de wallpapers guardados (máx 20) |
| `bump` | u8 | Bump de la PDA |

## Instrucciones Disponibles

**Crea el perfil on-chain del artista. Solo se puede ejecutar una vez por wallet:**
```
create_creator(nombre: String)
```

**Lista un nuevo wallpaper en el marketplace. Requiere tener perfil de creador:**
```
list_wallpaper(titulo: String, juego: String, precio: u64, total: u64, uri: String)
```

**Añade un wallpaper a la wishlist del usuario. Crea la cuenta wishlist si no existe:**
```
add_to_wishlist()
```

**Elimina un wallpaper de la wishlist del usuario:**
```
remove_from_wishlist()
```

**Cierra el listado de un wallpaper y devuelve el rent al artista:**
```
close_listing()
```

## Requisitos Previos

- **Solana Playground**: Plataforma web para compilar y desplegar el programa sin instalaciones locales. Disponible en [beta.solpg.io](https://beta.solpg.io).
- **Wallet de Solana**: Configura una wallet con fondos en Devnet. Solana Playground puede generarla automáticamente.
- **Fondos de Devnet**: Necesitarás SOL en Devnet para pagar el gas de las transacciones. Solana Playground tiene airdrop automático.

## Cómo Usar el Programa

### 1. Compilar el Programa

Abre el proyecto en Solana Playground y haz clic en el botón **"Build"**. Esto verifica que no haya errores de compilación y genera el IDL del programa.

### 2. Desplegar el Programa

Haz clic en el botón **"Deploy"** en Solana Playground. Esto publica el programa en Devnet y genera el Program ID automáticamente.

### 3. Crear un Perfil de Artista

Ejecuta la instrucción `create_creator` desde el `client.ts` cambiando la variable `ACCION`:

```typescript
const ACCION = "createCreator";
```

Haz clic en **"Run"**. Esto crea tu perfil on-chain con nombre de artista. Solo se puede crear una vez por wallet.

### 4. Listar un Wallpaper

Cambia la variable `ACCION` y ejecuta:

```typescript
const ACCION = "listWallpaper";
```

La consola mostrará la dirección PDA del wallpaper creado. Guarda esa dirección para los siguientes pasos.

### 5. Añadir a Wishlist

Pega la dirección del wallpaper en `WALLPAPER_ADDRESS` y ejecuta:

```typescript
const ACCION = "addToWishlist";
```

Crea la cuenta wishlist automáticamente si no existe y añade el wallpaper a la lista.

### 6. Eliminar de Wishlist

Con la misma dirección del wallpaper ejecuta:

```typescript
const ACCION = "removeFromWishlist";
```

Elimina el wallpaper de la wishlist del usuario.

### 7. Cerrar un Listado

Para cerrar un listado y recuperar el rent ejecuta:

```typescript
const ACCION = "closeListing";
```

Cierra la cuenta del wallpaper on-chain y devuelve el SOL bloqueado al artista.

### 8. Ver Cuentas On-Chain

Para leer todas las cuentas existentes en el programa ejecuta:

```typescript
const ACCION = "fetchAll";
```

Muestra en consola todos los perfiles de creador, wallpapers y wishlists almacenados en la blockchain.

## Códigos de Error

| Código | Nombre | Descripción |
|---|---|---|
| 6000 | `NoAutorizado` | La wallet no tiene permiso para esta acción |
| 6001 | `NombreLargo` | El nombre supera los 50 caracteres |
| 6002 | `EdicionesInvalidas` | El número de ediciones debe ser mayor a 0 |
| 6003 | `PrecioInvalido` | El precio debe ser mayor a 0 |
| 6004 | `YaEnWishlist` | El wallpaper ya está en la wishlist |
| 6005 | `NoEnWishlist` | El wallpaper no está en la wishlist |
| 6006 | `WishlistLlena` | La wishlist alcanzó el límite de 20 wallpapers |
| 6007 | `NoEnVenta` | El wallpaper no está disponible para esta acción |

## Notas Importantes

**Validaciones:** La función `create_creator` valida que el nombre no supere los 50 caracteres. La función `list_wallpaper` valida que el precio y las ediciones sean mayores a 0. La función `add_to_wishlist` valida que el wallpaper no esté duplicado en la lista y que no se supere el límite de 20 items.

**PDAs:** Todas las cuentas son PDAs (Program Derived Addresses), lo que significa que son direcciones únicas derivadas matemáticamente sin llaves privadas. Esto garantiza que solo el programa puede escribir en ellas.

**Índice como seed:** Cada wallpaper usa el índice actual del creador como seed para derivar su PDA. Esto evita colisiones entre wallpapers con el mismo título.

**Rent:** Al crear cuentas en Solana se bloquea una pequeña cantidad de SOL como depósito (rent). Al cerrar un listado con `close_listing`, ese SOL se devuelve al artista.

**Límite de wishlist:** La wishlist está limitada a 20 wallpapers porque el espacio de la cuenta se reserva al momento de crearla y no puede crecer automáticamente.

**Red:** Este programa está desplegado en **Solana Devnet**, una red de pruebas. Las cuentas en Devnet se reinician periódicamente.

## Limitaciones

**Wishlist fija:** El tamaño máximo de la wishlist es 20 wallpapers. Para soportar más items se requeriría `realloc` de Anchor, lo cual añade complejidad.

**Validación de URI:** La función `list_wallpaper` no valida que la URI sea una URL válida o que la imagen exista. Queda a criterio del artista proporcionar una URI correcta.

**Sin transferencia de SOL:** Este marketplace gestiona el listado y la wishlist de wallpapers, pero no incluye una instrucción de compra con transferencia de SOL entre usuarios.

---

## Autor

Desarrollado por **Cristofer Luna** como proyecto del Solana Bootcamp — WayLearn Latam.