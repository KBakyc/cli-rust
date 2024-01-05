@Echo off

echo Removing all "target" directories ...
rem fd -I "^target$" -td -X rd /s /q
for /r %1 %%d in (target) do if exist "%%d" (
  echo %%d
  rd /s /q "%%d"
)
echo Done.
