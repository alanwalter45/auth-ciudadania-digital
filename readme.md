# Ciudadania Digital

Implementación del servicio de autenticacion con ciudadanía digital.

#### Documentación

https://developer.ciudadaniadigital.bo/

Leer los apartados (Terminología, Autenticación, Autorización, Endpoints Utiles)

#### Discovery

https://proveedor.ciudadania.demo.agetic.gob.bo/.well-known/openid-configuration

#### Proyecto de prueba

```sh
# Descargar proyecto
git clone https://github.com/alanwalter45/auth-ciudadania-digital
# Registrar las variables de entorno en el archivo .env tomar de ejemplo el archivo .env.example
vim .env
# Crear ejecutable
cargo build --release
# Ejecutar aplicación
./target/release/auth
```

#### Pasos Basicos

-   Ingresar a [auth.chuquisaca.gob.bo](https://auth.chuquisaca.gob.bo)
-   Utilizar los usuarios de prueba del archivo (csv) proveido por la agetic en el apartado de configuracion del servicio.
-   Verificar codigo vìa [yopmail.com](https://yopmail.com/en/wm) salvo que utilice la opcion <u>Dispositivo de confianza</u>
-   Apartados a consumir
    -   Obtener Token
    -   Obtener informacion de usuario
    -   Refrescar Token
    -   Introspeccion
    -   Cerrar Sesion

#### Swagger

https://auth.chuquisaca.gob.bo/api-doc/

#### Front

https://github.com/alanwalter45/auth-ciudadania-digital-front

Autor

> alanwalter45@gmail.com
