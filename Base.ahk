﻿#NoEnv
#NoTrayIcon
#SingleInstance IGNORE
#WinActivateForce
#Persistent
#MaxHotkeysPerInterval 99000000
#HotkeyInterval 99000000
#KeyHistory
FileVersion=2.0.8.9
ScriptName=Devers
StartTime:=A_TickCount
IfNotExist, %A_MyDocuments%\%ScriptName%
FileCreateDir, %A_MyDocuments%\%ScriptName%
SetWorkingDir, %A_MyDocuments%\%ScriptName%
IfExist, settings.ini
{
	IniRead, InstalledFileVersion, build.ini, build, FileVersion, 1.0
	If % VersionCompare(FileVersion, InstalledFileVersion)=1
		GoSub UpdateFiles
}
#Include %A_ScriptDir%
GoSub defaultsettings
GoSub MenuInit
GoSub RunScript
return

;=====------======------=====------======------=====------======------=====------======------=====------======------=====------======------
;=====------======------=====------======------=====------======--MAIN FUNCTION--======------======------=====------======------=====------
;=====------======------=====------======------=====------======------=====------======------=====------======------=====------======------

RunScript: ; This is the beginning of the script
GoSub VolTR
GoSub IniReads
IniRead, LockPwHash, settings.ini, settings, LockPwHash ; keep in a separate thread with passwordhash decryption
if LockPwHash!=ERROR || LockPwHash ; keep in a separate thread with passwordhash decryption
LockPw:=Crypt.Encrypt.StrDecrypt(LockPwHash,"KktgC3l0wR",7,3) ; keep in a separate thread with passwordhash decryption
SetTimer, CheckBreakLoop, -2500
SetTimer, RestartCheck, 800
SetTimer, AllowUpdateCheckInMenu, 3600000
IfExist, settings.ini
{
	If RunAsAdmin && !LockAfterRestart
		RunAsAdmin()

	If Reloaded=1
	{
		If !A_IsCompiled
		RSNotify("Reloaded")
		IniDelete, settings.ini, settings, Reloaded
	}
	If CreateATask=1 && (A_IsCompiled)
	{
			Run, %comspec% /c SchTasks /SC ONLOGON /F /Create /TN RSRunScript /RL HIGHEST /TR "%A_ScriptDir%\launcher.exe, , HIDE
			IniWrite, 0, settings.ini, settings, CreateATask
	}
	
	WinSet, Transparent, %TransparentStartMenu%, ahk_class Shell_TrayWnd
	SetTimer, KeepTrans, 12000
	
	OnExit, ExitAppL

	; How to use the plugin system :
	; Just create a file with the extension .ahk in the Script directory and it will run next time you restart the script
	IfExist, *.ahk
	{
		IfNotExist, Extensions
			FileInstall, Base/Extensions, %A_MyDocuments%\%ScriptName%\Extensions, 1

		IfNotExist, Libraries
			FileCreateDir, Libraries
		IfNotExist, %A_WorkingDir%/Libraries/Functions.lib
			FileInstall, Libraries/Functions.lib, %A_MyDocuments%\%ScriptName%\Libraries\Functions.lib, 1
		IfNotExist, %A_WorkingDir%/Libraries/Library.lib
			FileInstall, Libraries/Library.lib, %A_MyDocuments%\%ScriptName%\Libraries\Library.lib, 1

		Loop, Files, *.ahk
		{
			SplitPath, A_LoopFileLongPath,,,, PluginName

			If RegExMatch(PluginName, "\W")
			{
				PluginName:=StrReplace(PluginName, A_Space, "_")
				PluginName := RegExReplace(PluginName, "\W")
				FileMove, %A_LoopFileLongPath%, %PluginName%.ahk, 1
			}

			IniRead, PluginState, settings.ini, plugins, PluginState%PluginName%, 1
			If(PluginState=1)
				Run, %A_MyDocuments%\%ScriptName%\Extensions %A_MyDocuments%\%ScriptName%\%PluginName%.ahk,,, PID%PluginName% ; PluginID is the PID for the process. Required when you need to close/uninstall the program.
		}
	}

	GoSub InstallFiles
	If !LockAfterRestart && (A_IsCompiled) && !AfterInstallation
			SetTimer, RSRunScript, -25
	
	If LockAfterRestart=1
	{
		GoSub RSGuard
		If RunAsAdmin
			RunAsAdminAfterLock:=1
	}

	If hotkeysInFullscreen=2
		SetTimer, HotkeysSuspendCheck, 500

	LogTime(0)
	GoSub ButtonsLabel
	If AfterInstallation=1
		GoSub ScriptFunctions
}
IfNotExist, settings.ini
	Gosub Installation
return

Installation:
	FirstRun:=true
	FileCreateDir, %A_MyDocuments%\%ScriptName%
	SetWorkingDir, %A_MyDocuments%\%ScriptName%
	RSNotify("Installing",,750)
	sleep 1350
	GoSub InstallFiles
	Menu, Tray, Icon
	Menu, Tray, Icon, OptionsIcon.ico
	GoSub AdminPrompt
return

InstallFiles:
	IfNotExist, launcher.exe
		FileInstall, Base/launcher.exe, %A_MyDocuments%\%ScriptName%\launcher.exe, 1
	IfNotExist, RS.png
		FileInstall, Base/RS.png, %A_MyDocuments%\%ScriptName%\RS.png, 1
	IfNotExist, RSAnimation.mp4
		FileInstall, Base/RSAnimation.mp4, %A_MyDocuments%\%ScriptName%\RSAnimation.mp4, 1
	IfNotExist, RSStopped.ico
		FileInstall, Base/RSStopped.ico, %A_MyDocuments%\%ScriptName%\RSStopped.ico, 1
	IfNotExist, OptionsIcon.ico
		FileInstall, Base/OptionsIcon.ico, %A_MyDocuments%\%ScriptName%\OptionsIcon.ico, 1
	IfNotExist, Extensions
			FileInstall, Base/Extensions, %A_MyDocuments%\%ScriptName%\Extensions, 1
	IfNotExist, Tip_small.png
		FileInstall, Base/Tip_small.png, %A_MyDocuments%\%ScriptName%\Tip_small.png, 1
	IfNotExist, Tip_medium.png
		FileInstall, Base/Tip_medium.png, %A_MyDocuments%\%ScriptName%\Tip_medium.png, 1
	IfNotExist, Tip_large.png
		FileInstall, Base/Tip_large.png, %A_MyDocuments%\%ScriptName%\Tip_large.png, 1
	IniWrite, %FileVersion%, build.ini, build, FileVersion
return

;=====------======------=====------======------=====------======------=====------======------=====------======------=====------======------
;=====------======------=====------======------=====------======------LABELS END-======------=====------======------=====------======------
;=====------======------=====------======------=====------======------=====------======------=====------======------=====------======------

;--------------------------------------------------------------------------------------------------------------
#Include Libraries/Tip.lib
#Include Libraries/Hotkeys.lib
#Include Libraries/IniReads.lib
#Include Libraries/Functions.lib
#Include Libraries/Library.lib
#Include Libraries/RandomLabels.lib
#Include Libraries/updater.lib
#Include Libraries/RSGuard.lib
#Include Libraries/RSWeb.lib
#Include Libraries/Menu.lib
#Include Libraries/Voltr.lib