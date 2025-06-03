# Ciudadanía Digital

Implementación del <u>servicio de autenticación</u> con ciudadanía digital.

#### Tecnología

Rust https://www.rust-lang.org/

```sh
# Instalar rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Documentación AGETIC

https://developer.ciudadaniadigital.bo/docs/servicios/autenticacion

- Leer apartados del servicio (Terminología, Autenticación, Autorización, Endpoints Utiles).
- Registrar un proyecto de prueba para utilizar el API del servicio de autenticación.

#### Discovery

Proveedor de prueba
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

#### Pasos Básicos

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

Los archivos <strong>Dockerfile.Base y Dockerfile.Launch</strong> son para crear un ambiente específico de producción (Ejemplo a ser ajustado acorde al requerimiento).

```sh
# Crear imagen base con rust
$ docker build -f Dockerfile.Base --no-cache=true -t rust-bullseye .
# Generar ejecutable para entorno de producción
$ docker build -f Dockerfile.Launch --no-cache=true -t rust-cargo-bullseye .
# Volumen para acceder al ejecutable
$ docker run -it --rm -v rust_volume:/app rust-cargo-bullseye
```

> Autor : alanwalter45@gmail.com
