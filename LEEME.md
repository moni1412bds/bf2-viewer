# BF2 Visor (Tauri) — cómo compilarlo con GitHub

## Pasos (una sola vez)
1. Entrá a https://github.com y creá una cuenta gratis (si no tenés).
2. Botón **New repository** → nombre `bf2-visor` → **Create repository**.
3. En el repositorio: **uploading an existing file** → arrastrá **el contenido** de esta carpeta (src, src-tauri, package.json, .gitignore) → **Commit changes**.
4. La carpeta oculta `.github` a veces no se sube arrastrando. Si en la pestaña **Actions** no aparece "Compilar BF2 Visor":
   **Add file → Create new file**, nombre `.github/workflows/build.yml`, pegá el contenido del archivo `.github/workflows/build.yml` de este zip y **Commit**.
5. Pestaña **Actions** → "Compilar BF2 Visor" → **Run workflow**. Tarda ~10 minutos.
6. Al terminar (tilde verde), entrá a esa ejecución y bajá de **Artifacts**:
   - `BF2-Visor-instalador`: instalador. Registra "Abrir con" para .con, .tweak, .bundledmesh, .staticmesh y .skinnedmesh.
   - `BF2-Visor-portable`: el .exe suelto, sin instalar (sin "Abrir con" automático).

## Uso
- Doble clic en un .con / malla (con el instalador) o botón **Abrir archivo…** dentro del programa.
- Busca solo los archivos relacionados en las subcarpetas del mod (como hacía bf2_bridge.py).
- **Guardar en disco** (editor .con/.tweak y editor de materiales): escribe sobre el archivo y crea una copia `.bak` la primera vez.
- Necesita WebView2 (ya viene con Windows 10/11 actualizado).

## bf2_visor.ini
- Guarda la ruta del último `.ske` y `.skinnedmesh` usados (se actualiza solo al abrir un archivo o cambiar esos selectores).
- Se crea junto al `.exe`; si esa carpeta no permite escribir, usa la carpeta de configuración del usuario.
- Formato (se puede editar a mano):
  ```
  [ultimo]
  ske=C:\ruta\1p_setup.ske
  skinnedmesh=C:\ruta\us_light_soldier.skinnedmesh
  ```
- Si una ruta ya no existe, el programa busca otra automáticamente en las carpetas del mod.

## Si falla la compilación
Entrá a la ejecución en rojo → paso que falló → copiá las últimas líneas del log y pasámelas.
