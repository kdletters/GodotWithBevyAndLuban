set WORKSPACE=.

set LUBAN_DLL=%WORKSPACE%\Tools\Luban\Luban.dll
set CONF_ROOT=%WORKSPACE%

dotnet %LUBAN_DLL% ^
    -t all ^
    -c rust-bin ^
    -d bin ^
    -d json ^
    --conf %CONF_ROOT%\luban.conf ^
    -x outputCodeDir=..\rust\gen ^
    -x outputDataDir=..\godot\data\bytes ^
    -x json.outputDataDir=%WORKSPACE%\json^
    -x pathValidator.rootDir=..\godot
@REM     -x l10n.provider=default ^
@REM     -x l10n.textFile.path=*@%WORKSPACE%\DataTables\Datas\l10n\texts.json ^
@REM     -x l10n.textFile.keyFieldName=key

pause