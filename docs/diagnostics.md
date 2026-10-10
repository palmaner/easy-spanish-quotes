# Diagnóstico del prototipo

`easy-spanish-quotes status` y `easy-spanish-quotes doctor` solo leen el estado;
no instalan, activan, reparan ni elevan privilegios.

Cuando existe un recibo, `installation.state` combina lo guardado con las
observaciones actuales del archivo, el registro y la inscripción del propietario.
`installation.journal_state` conserva el estado guardado. Un estado `enabled`
significa que la inscripción está comprobada para ese usuario; la selección
activa y la persistencia tras reiniciar siguen sin estar demostradas.

Un archivo ausente o modificado, un registro cambiado, una operación interrumpida
o una lectura que no se puede verificar produce `repair-required`. La inspección
desde otro usuario no usa sus perfiles como prueba de la inscripción del dueño:
`enrolled` vale `null` y `owner_matches` vale `false`.

`findings` contiene códigos estables para investigar:

| Código | Significado |
|---|---|
| artifact-modified / artifact-unreadable | El archivo cambió o no puede verificarse |
| registration-modified / registration-unreadable | El registro cambió o no puede verificarse |
| owner-enrollment-unverified | No se pudo verificar la inscripción del propietario |
| journal-observation-mismatch | Las observaciones no respaldan el estado guardado |
| interrupted-operation | Queda una intención sin completar |
| receipt-cleanup-required | El archivo pendiente de reinicio ya no está, pero queda el recibo |

`pending-reboot` requiere que el archivo pendiente siga verificado, que el
registro ya no exista y que el propietario no tenga el perfil inscrito. Cuando
el archivo desaparece, el diagnóstico pide revisar y cerrar el recibo mediante
el protocolo de la VM. No afirma que toda la eliminación haya terminado.

`status` devuelve 0 cuando obtiene un informe, aunque incluya hallazgos.
`doctor` y los resultados de operaciones nativas devuelven 7 si requieren
reparación y 8 si requieren reinicio. Los errores que impiden obtener el informe
siguen devolviendo 3 en este prototipo; la clasificación completa de errores de
la especificación todavía está pendiente. `doctor.native_validation` permanece
en `pending`: no convierte las pruebas automatizadas en validación de Windows.

Los argumentos se validan por completo antes de ejecutar el comando. Una opción
desconocida, repetida o incompatible devuelve 2; un comando no implementado
devuelve 3. `--owner` requiere un valor SID y no puede consumir otra opción.
`--version` muestra la versión experimental. No existe una opción `--force`.
