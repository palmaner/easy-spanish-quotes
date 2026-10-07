# easy-spanish-quotes

Escribe comillas españolas **« »** sin mantener una aplicación abierta.

Este proyecto está en desarrollo. El primer prototipo usa una distribución
nativa de Windows basada en el teclado español de España:

| Combinación | Carácter |
|---|---|
| AltGr + Z | « |
| AltGr + X | » |

El teclado original se conserva. La primera versión pública incluirá una
interfaz gráfica en español para activar, desactivar y quitar la configuración.
macOS y Linux forman parte del plan; todavía no se anuncian como compatibles.

## Estado

Consulta [los resultados de validación](docs/validation.md) antes de probarlo.
Una compilación correcta no demuestra que la instalación persista tras reiniciar.
Las pruebas de instalación se realizarán primero en máquinas desechables.
Todavía no hay una versión pública ni un instalador firmado.

## Desarrollo

El núcleo y la herramienta de Windows se escriben en Rust. La distribución
del teclado usa tablas nativas en C y requiere herramientas de compilación;
el usuario final no tendrá que instalarlas.

```powershell
cargo test --workspace
cargo run -- status
```

El documento [IMPLEMENTATION.md](IMPLEMENTATION.md) contiene la especificación,
las fuentes, los hitos y los criterios para publicar una versión.

## Licencia

Apache-2.0 para el código propio. Las dependencias y cualquier material de
terceros conservan sus licencias; véase [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
