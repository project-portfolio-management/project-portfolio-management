# Tutorial

*Pasos para ejecutar la demostración en una computadora local y revisar una cartera de muestra.*

La demostración es una copia completa del software, cargada con datos de una organización ficticia, la Northgate Transport Authority. Todas las personas, planes, cifras y fechas son ficticios, y los datos se pueden modificar libremente.

**Nota de seguridad.** La demostración confía en una llave de prueba publicada. No debe ejecutarse en una red compartida ni usarse con datos reales.

## Requisitos

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) con Compose

## Paso 1: descargar el código

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Paso 2: iniciar la demostración

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Este comando compila el software, inicia la base de datos, carga la cartera ficticia e inicia la interfaz. La primera compilación tarda más que las siguientes.

## Paso 3: iniciar sesión

Abra <http://localhost:3000/es-001/signin>, deje la dirección como `example@example.com` y pulse **Sign in to the demonstration**. No se envía ningún correo electrónico. La demostración conserva las etiquetas en inglés de su interfaz, y este tutorial las cita tal como aparecen.

La primera página es el **recorrido de la demostración**, que enumera cada función por grupo, con una descripción y un enlace. Un banner amarillo en cada página indica que los datos son ficticios.

## Paso 4: revisar la cartera

Desde el recorrido, abra **Plans** para ver la cartera, sus programas y sus proyectos. Después abra el **Portfolio dashboard**.

## Paso 5: revisar un proyecto

Abra el proyecto **North Line interlocking** desde el grupo de entrega del recorrido. Revise su diagrama de **Gantt**, su **Critical path** y su **Earned value**.

## Paso 6: revisar el trabajo personal

Abra **My work** y **My kanban board**. La sesión está iniciada como Example Person, que tiene tareas abiertas.

## Paso 7: revisar la capacidad, el control de inicio y los informes

Abra **Capacity heatmap** para ver los recursos con exceso de asignación. Abra **Start queue**, donde el límite de trabajo en curso ha retenido un programa. Abra **Director packs**, el resumen mensual que se congela una vez emitido.

## Paso 8: detener la demostración

Este comando detiene todos los servicios y elimina los datos de la demostración:

```sh
podman compose --profile demo down -v
```

## Siguientes pasos

- [Guía de instalación](../../../../INSTALL.md), con el inicio de sesión y los proveedores de identidad
- [Fichas por función](../../for/) para los puestos directivos
- [Guía de la demostración](../../../../operations/demo.md)
- [Lista de funciones](../funciones/index.md)

Los problemas se pueden reportar en el [sistema de seguimiento de incidencias](https://github.com/project-portfolio-management/project-portfolio-management/issues).
