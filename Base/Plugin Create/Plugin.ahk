﻿#NoEnv
#NoTrayIcon
#SingleInstance FORCE
SendMode Input
SetWorkingDir %A_ScriptDir%
SplitPath, A_ScriptName,,,, PluginName
Menu, Tray, Icon, RSIcon.ico
global dpi:=DpiFactor() ; required if you wish to use RSNotify(needs the 3 libraries included)
IniRead, Hotkey, plugin-settings.ini, %PluginName%, key, UNASSIGNED
If Hotkey=UNASSIGNED
    GoSub ChooseHotkey
else
    GoSub AssignHotkey
return

; ----------- User functions here -------------

Function:
return

; -------------- HOTKEY PROMPT ----------------
ChooseHotkey:
	width=235
	height=80
	Gui, ChooseHotkey:New, +hwndChooseHotkey
	Gui, +LastFound
	WinSet, Transparent, 0
	Gui, Margin, 0, 0
	Gui, -Caption
	Gui, Color, 008383
	Gui, Font, s10 Bold, Tahoma
	Opt1 := [6, 0x008383, 0x008383, "White"]
	Opt2 := [ , 0x008383, 0x00a3a3, 0xffffff]
	Opt4 := [0, 0xC0A0A0A0, , 0xC0606000]
	Gui, Add, Progress, % "x-1 y-1 w" width " h26 Background1F2326 Disabled hwndHPROG"
	Control, ExStyle, -0x20000, , ahk_id %HPROG% ; propably only needed on Win XP
	Gui, Add, Text, x0 y4 w%width% Center BackgroundTrans +0x200 c228a96, Choose your desired hotkey!
	Gui, Add, Hotkey, % "x" Width/10 " y+18 w" (Width/2) " Center vHotkey ",
    Gui, Add, Button, x+12 w70 h25 +default gSubmitButton hwndBut1 +Center, Ready
	ImageButton.Create(But1, Opt1, Opt2, "", Opt4)
	width:=width*dpi
	height:=height*dpi
	Gui, Show, w%width% h%height%, Desired hotkey?
	WinSet, Region, 0-0 w%width% h%height% r6-6, ahk_id %ChooseHotkey%
	FadeIn(ChooseHotkey)
return
SubmitButton:
    Gui, Submit, NoHide
    If (Hotkey!="None" && Hotkey!="" && Hotkey!=UNASSIGNED)
    {
        FadeOut(ChooseHotkey)
        GoSub AssignHotkey
    }
    else
        RSNotify("Not possible")
return
ChooseHotkeyGuiEscape:
    FadeOut(ChooseHotkey)
    ExitApp
return
AssignHotkey:
    IniWrite, %Hotkey%, plugin-settings.ini, %PluginName%, key
    Hotkey, %Hotkey%, Function
return

#Include Libraries\Functions.lib
#Include Libraries\RSNotify.lib
#Include Libraries\Library.lib