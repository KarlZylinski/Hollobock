call rustc -Llib/ hollobock.rs -o bin/hollobock.exe
if errorlevel 1 (
   echo Build failed
   exit /b %errorlevel%
)
cd bin
start ./hollobock.exe
cd ..