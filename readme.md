# Ciudadania Digital

Implementación del servicio de autenticación con ciudadanía digital.

#### Documentación

https://developer.ciudadaniadigital.bo/docs/servicios/autenticacion

- Leer los apartados del servicio (Terminología, Autenticación, Autorización, Endpoints Utiles).
- Registrar un proyecto de prueba para utilizar el API del servicio de autenticación.

#### Discovery

URL del proveedor de prueba
https://proveedor.ciudadania.demo.agetic.gob.bo/.well-known/openid-configuration

#### Proyecto

```sh
# Descargar proyecto
$ git clone https://github.com/alanwalter45/auth-ciudadania-digital
# Registrar las variables de entorno en el archivo ".env" desde el archivo ".env.example"
$ cp .env.example .env
$ vim .env
# Crear ejecutable
$ cargo build --release
# Ejecutar aplicación
$ ./target/release/auth-gadch
```

#### Pasos Basicos

-   Utilizar los usuarios de prueba del archivo (csv) proveido por la AGETIC en el apartado de configuración del servicio.
-   Verificar codigo vía [yopmail.com](https://yopmail.com/en/wm) salvo que utilice la opción <u>Dispositivo de confianza</u>
-   Apartados a consumir de auth-ciudadania-digital:
    -   Generar URL para autenticación.
    -   Obtener Token.
    -   Obtener información de usuario.
    -   Refrescar Token.
    -   Introspección.
    -   Generar URL para cerrar sesión.

#### Swagger

Documentación del API del aplicativo

https://autenticacion-beta.chuquisaca.gob.bo/swagger-ui/

#### Front

https://github.com/alanwalter45/auth-ciudadania-digital-front

#### Docker

Los archivos Dockerfile.Base y Dockerfile.Launch son para crear un ambiente específico para un entorno de  producción (Ejemplo a ser acondicionado acorde al requerimiento).

```sh
# Crear imagen base con rust
docker build -f Dockerfile.Base --no-cache=true -t rust-bullseye .
# Generar ejecutable para entorno de producción
docker build -f Dockerfile.Launch --no-cache=true -t rust-cargo-bullseye .
# Volumen para acceder al ejecutable
docker run -it --rm -v rust_volume:/app rust-cargo-bullseye
```

> Autor : alanwalter45@gmail.com
