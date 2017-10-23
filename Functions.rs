﻿
;--------------------------------------------------------------------------------------------------------------

RSNotify(Text, controlwidth=0, fontsize=13, TitleSpace=0, Title="", NewY=9, Timed=1250) ;RSNotify
{
	global DontPause=1
	global SmartFade=0
	Gui, RSNotify:Destroy
	Gui,RSNotify:New, +hwndRSNotify
	Gui, +LastFound +ToolWindow
	WinSet, Transparent, 0
	Gui, Color, 008383
	Gui, Margin, 0, 0
	Gui, Font, s10 Bold w640, Verdana
	whc:=47
	
	If controlwidth=0
	{
	widthcontrolled:=false
	controlwidth=105
	}
	Else
	widthcontrolled:=true
	
	If (StrLen(Text)>9) && !widthcontrolled
	controlwidth:=controlwidth+((StrLen(Text)-9)*10)+10
	
	vhc:=A_ScreenHeight-100
	
	If TitleSpace
	{
	whc=48
	NewY=19
	controlwidth+=6
	Gui, Add, Progress, % "x-1 y-1 w" controlwidth " h22 Background1F2326 Disabled hwndHPROG"
	Control, ExStyle, -0x20000, , ahk_id %HPROG% ; propably only needed on Win XP
	Gui, Add, Text, % "x0 y1 w" controlwidth " r1 BackgroundTrans Center 0x200 gGuiMove c248a96", %Title%
	}
	averagex:=A_ScreenWidth-130-(controlwidth/2)
	Gui, Font, s%fontsize% w720, Verdana
	Gui, -Border -Caption +AlwaysOnTop +ToolWindow +E0x20
	Gui, Add, Text,cffffff x0 w%controlwidth% y%NewY% +Center, %Text%
	WinSet, Region, 0-0 w%controlwidth% h%whc% r3-3
	Gui, Show, x%averagex% y%vhc% h%whc% w%controlwidth% NoActivate
	GoSub SmartAppear
	SetTimer, SmartFade, -%Timed%
return
}

SmartAppear:
	SmartFade:=SmartFade+8
	WinSet, Transparent, %SmartFade%, ahk_id %RSNotify%
	SetTimer, SmartAppear, -8
	If (SmartFade>204)
	SetTimer, SmartAppear, Off
return
	
SmartFade:
		SmartFade:=SmartFade-8
		WinSet, Transparent, %SmartFade%, ahk_id %RSNotify%
		SetTimer, SmartFade, -8
		If (SmartFade<8){
			SetTimer, SmartFade, Off
			Gui, RSNotify:Destroy
			If TryPause
			Pause, On
			DontPause=0
		}
return

;--------------------------------------------------------------------------------------------------------------

RapidHotkey(keystroke, times="2", delay=0.2, IsLabel=0)
{
	Pattern := Morse(delay*1000)
	If (StrLen(Pattern) < 2 and Chr(Asc(times)) != "1")
		Return
	If (times = "" and InStr(keystroke, """"))
	{
		Loop, Parse, keystroke,""	
			If (StrLen(Pattern) = A_Index+1)
				continue := A_Index, times := StrLen(Pattern)
	}
	Else if (RegExMatch(times, "^\d+$") and InStr(keystroke, """"))
	{
		Loop, Parse, keystroke,""
			If (StrLen(Pattern) = A_Index+times-1)
				times := StrLen(Pattern), continue := A_Index
	}
	Else if InStr(times, """")
	{
		Loop, Parse, times,""
			If (StrLen(Pattern) = A_LoopField)
				continue := A_Index, times := A_LoopField
	}
	Else if (times = "")
		continue := 1, times := 2
	Else if (times = StrLen(Pattern))
		continue = 1
	If !continue
		Return
	Loop, Parse, keystroke,""
		If (continue = A_Index)
			keystr := A_LoopField
	Loop, Parse, IsLabel,""
		If (continue = A_Index)
			IsLabel := A_LoopField
	hotkey := RegExReplace(A_ThisHotkey, "[\*\~\$\#\+\!\^]")
	IfInString, hotkey, %A_Space%
		StringTrimLeft, hotkey,hotkey,% InStr(hotkey,A_Space,1,0)
	backspace := "{BS " times "}"
	keywait = Ctrl|Alt|Shift|LWin|RWin
	Loop, Parse, keywait, |
		KeyWait, %A_LoopField%
	If ((!IsLabel or (IsLabel and IsLabel(keystr))) and InStr(A_ThisHotkey, "~") and !RegExMatch(A_ThisHotkey
	, "i)\^[^\!\d]|![^\d]|#|Control|Ctrl|LCtrl|RCtrl|Shift|RShift|LShift|RWin|LWin|Alt|LAlt|RAlt|Escape|BackSpace|F\d\d?|"
	. "Insert|Esc|Escape|BS|Delete|Home|End|PgDn|PgUp|Up|Down|Left|Right|ScrollLock|CapsLock|NumLock|AppsKey|"
	. "PrintScreen|CtrlDown|Pause|Break|Help|Sleep|Browser_Back|Browser_Forward|Browser_Refresh|Browser_Stop|"
	. "Browser_Search|Browser_Favorites|Browser_Home|Volume_Mute|Volume_Down|Volume_Up|MButton|RButton|LButton|"
	. "Media_Next|Media_Prev|Media_Stop|Media_Play_Pause|Launch_Mail|Launch_Media|Launch_App1|Launch_App2"))
		Send % backspace
	If (WinExist("AHK_class #32768") and hotkey = "RButton")
		WinClose, AHK_class #32768
	If !IsLabel
		Send % keystr
	else if IsLabel(keystr)
		Gosub, %keystr%
	Return
}	
Morse(timeout = 400) { ;by Laszo -> http://www.autohotkey.com/forum/viewtopic.php?t=16951 (Modified to return: KeyWait %key%, T%tout%)
   tout := timeout/1000
   key := RegExReplace(A_ThisHotKey,"[\*\~\$\#\+\!\^]")
   IfInString, key, %A_Space%
		StringTrimLeft, key, key,% InStr(key,A_Space,1,0)
	If Key in Shift,Win,Ctrl,Alt
		key1:="{L" key "}{R" key "}"
   Loop {
      t := A_TickCount
      KeyWait %key%, T%tout%
		Pattern .= A_TickCount-t > timeout
		If(ErrorLevel)
			Return Pattern
    If key in Capslock,LButton,RButton,MButton,ScrollLock,CapsLock,NumLock
      KeyWait,%key%,T%tout% D
    else if Asc(A_ThisHotkey)=36
		KeyWait,%key%,T%tout% D
    else
      Input,pressed,T%tout% L1 V,{%key%}%key1%
	If (ErrorLevel="Timeout" or ErrorLevel=1)
		Return Pattern
	else if (ErrorLevel="Max")
		Return
   }
}

;--------------------------------------------------------------------------------------------------------------

winfade(w:="",t:=128,i:=1,d:=10) { ; by joedf  -  http://ahkscript.org/boards/viewtopic.php?f=6&t=512
	w:=(w="")?("ahk_id " WinActive("A")):w
	t:=(t>255)?255:(t<0)?0:t
	WinGet,s,Transparent,%w%
	s:=(s="")?255:s ;prevent trans unset bug
	WinSet,Transparent,%s%,%w% 
	i:=(s<t)?abs(i):-1*abs(i)
	while(k:=(i<0)?(s>t):(s<t)&&WinExist(w)) {
		WinGet,s,Transparent,%w%
		s+=i
		WinSet,Transparent,%s%,%w%
		sleep %d%
	}
}
return

;--------------------------------------------------------------------------------------------------------------

ProcessExist(Name){
	Process,Exist,%Name%
	return Errorlevel
}

;--------------------------------------------------------------------------------------------------------------

GetActiveBrowserURL() {
	WinGetClass, sClass, A
	If sClass In Chrome_WidgetWin_1,Chrome_WidgetWin_0,Maxthon3Cls_MainFrm
		Return GetBrowserURL_ACC(sClass)
	Else
		Return GetBrowserURL_DDE(sClass) ; empty string if DDE not supported (or not a browser)
}

;"GetBrowserURL_DDE" adapted from DDE code by Sean, (AHK_L version by maraskan_user)
; Found at http://autohotkey.com/board/topic/17633-/?p=434518

GetBrowserURL_DDE(sClass) {
	WinGet, sServer, ProcessName, % "ahk_class " sClass
	StringTrimRight, sServer, sServer, 4
	iCodePage := A_IsUnicode ? 0x04B0 : 0x03EC ; 0x04B0 = CP_WINUNICODE, 0x03EC = CP_WINANSI
	DllCall("DdeInitialize", "UPtrP", idInst, "Uint", 0, "Uint", 0, "Uint", 0)
	hServer := DllCall("DdeCreateStringHandle", "UPtr", idInst, "Str", sServer, "int", iCodePage)
	hTopic := DllCall("DdeCreateStringHandle", "UPtr", idInst, "Str", "WWW_GetWindowInfo", "int", iCodePage)
	hItem := DllCall("DdeCreateStringHandle", "UPtr", idInst, "Str", "0xFFFFFFFF", "int", iCodePage)
	hConv := DllCall("DdeConnect", "UPtr", idInst, "UPtr", hServer, "UPtr", hTopic, "Uint", 0)
	hData := DllCall("DdeClientTransaction", "Uint", 0, "Uint", 0, "UPtr", hConv, "UPtr", hItem, "UInt", 1, "Uint", 0x20B0, "Uint", 10000, "UPtrP", nResult) ; 0x20B0 = XTYP_REQUEST, 10000 = 10s timeout
	sData := DllCall("DdeAccessData", "Uint", hData, "Uint", 0, "Str")
	DllCall("DdeFreeStringHandle", "UPtr", idInst, "UPtr", hServer)
	DllCall("DdeFreeStringHandle", "UPtr", idInst, "UPtr", hTopic)
	DllCall("DdeFreeStringHandle", "UPtr", idInst, "UPtr", hItem)
	DllCall("DdeUnaccessData", "UPtr", hData)
	DllCall("DdeFreeDataHandle", "UPtr", hData)
	DllCall("DdeDisconnect", "UPtr", hConv)
	DllCall("DdeUninitialize", "UPtr", idInst)
	csvWindowInfo := StrGet(&sData, "CP0")
	StringSplit, sWindowInfo, csvWindowInfo, `"
	Return sWindowInfo2
}

GetBrowserURL_ACC(sClass) {
	global nWindow, accAddressBar
	If (nWindow != WinExist("ahk_class " sClass)) ; reuses accAddressBar if it's the same window
	{
		nWindow := WinExist("ahk_class " sClass)
		accAddressBar := GetAddressBar(Acc_ObjectFromWindow(nWindow))
	}
	Try sURL := accAddressBar.accValue(0)
	If (sURL == "") {
		sURL := accAddressBar.accDescription(0) ; Origin Chip support
		If (sURL == "") {
			WinGet, nWindows, List, % "ahk_class " sClass ; In case of a nested browser window as in CoolNovo
			If (nWindows > 1) {
				accAddressBar := GetAddressBar(Acc_ObjectFromWindow(nWindows2))
				sURL := accAddressBar.accValue(0)
			}
		}
	}
	If ((sURL != "") and (SubStr(sURL, 1, 4) != "http")) ; Chromium-based browsers omit "http://"
		sURL := "http://" sURL
	Return sURL
}

; "GetAddressBar" based in code by uname
; Found at http://autohotkey.com/board/topic/103178-/?p=637687

GetAddressBar(accObj) {
	Try If ((accObj.accName(0) != "") and IsURL(accObj.accValue(0)))
		Return accObj
	Try If ((accObj.accName(0) != "") and IsURL("http://" accObj.accValue(0))) ; Chromium omits "http://"
		Return accObj
	Try If (InStr(accObj.accDescription(0), accObj.accName(0)) and IsURL(accObj.accDescription(0))) ; Origin Chip support
		Return accObj
	For nChild, accChild in Acc_Children(accObj)
		If IsObject(accAddressBar := GetAddressBar(accChild))
			Return accAddressBar
}

IsURL(sURL) {
	Return RegExMatch(sURL, "^(?<Protocol>https?|ftp)://(?:(?<Username>[^:]+)(?::(?<Password>[^@]+))?@)?(?<Domain>(?:[\w-]+\.)+\w\w+)(?::(?<Port>\d+))?/?(?<Path>(?:[^/?# ]*/?)+)(?:\?(?<Query>[^#]+)?)?(?:\#(?<Hash>.+)?)?$")
}

; The code below is part of the Acc.ahk Standard Library by Sean (updated by jethrow)
; Found at http://autohotkey.com/board/topic/77303-/?p=491516

;--------------------------------------------------------------------------------------------------------------

PutUni(DataIn)
{
   SavedClip := ClipBoardAll

   ClipBoard =
   If RegExMatch(DataIn, "^[0-9a-fA-F]+$")
   {
      Loop % StrLen(DataIn) / 2
         UTF8Code .= Chr("0x" . SubStr(DataIn, A_Index * 2 - 1, 2))
   }
   Else
      UTF8Code := DataIn

   Send %UTF8Code%

   Sleep 50
   ClipBoard := SavedClip
   return
}

;--------------------------------------------------------------------------------------------------------------

RunAsAdmin() {
	Loop, %0%  ; For each parameter:
		{
		param := %A_Index%  ; Fetch the contents of the variable whose name is contained in A_Index.
		params .= A_Space . param
		}
	ShellExecute := A_IsUnicode ? "shell32\ShellExecute":"shell32\ShellExecuteA"
		
	if not A_IsAdmin
	{
		If A_IsCompiled
			DllCall(ShellExecute, uint, 0, str, "RunAs", str, A_ScriptFullPath , str, params , str, A_WorkingDir, int, 1)
		Else
			DllCall(ShellExecute, uint, 0, str, "RunAs", str, A_AhkPath, str, """" . A_ScriptFullPath . """" . A_Space . params, str, A_WorkingDir, int, 1)
		ExitApp
  }
}

;--------------------------------------------------------------------------------------------------------------

isWindowFullScreen( winTitle ) {
	;checks if the specified window is full screen
	
	winID := WinExist( winTitle )

	If ( !winID )
		Return false

	WinGet style, Style, ahk_id %WinID%
	WinGetPos ,,,winW,winH, %winTitle%
	; 0x800000 is WS_BORDER.
	; 0x20000000 is WS_MINIMIZE.
	; no border and not minimized
	Return ((style & 0x20800000) or winH < A_ScreenHeight or winW < A_ScreenWidth) ? false : true
}

;--------------------------------------------------------------------------------------------------------------

BlockKeyboardInputs(state = "On")
{
   static keys
   keys=Space,Enter,Tab,Esc,BackSpace,Del,Ins,Home,Up,Down,Left,Right,CtrlBreak,ScrollLock,PrintScreen,CapsLock
,Pause,AppsKey,NumLock,Numpad0,Numpad1,Numpad2,Numpad3,Numpad4,Numpad5,Numpad6,Numpad7,Numpad8,Numpad9,NumpadDot
,NumpadDiv,NumpadMult,NumpadAdd,NumpadSub,NumpadEnter,NumpadIns,NumpadEnd,NumpadDown,NumpadPgDn,NumpadLeft,NumpadClear
,NumpadRight,NumpadHome,NumpadUp,NumpadPgUp,NumpadDel,Media_Next,Media_Play_Pause,Media_Prev,Media_Stop,Volume_Down,Volume_Up
,Volume_Mute,Browser_Back,Browser_Favorites,Browser_Home,Browser_Refresh,Browser_Search,Browser_Stop,Launch_App1,Launch_App2
,Launch_Mail,Launch_Media,F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22
,1,2,3,4,5,6,7,8,9,0,a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,x,y,z
,²,&,é,",',(,-,è,_,ç,à,),=,$,£,ù,*,~,#,{,[,|,``,\,^,@,],},;,:,!,?,.,/,§,<,>,vkBC
   Loop,Parse,keys, `,
      Hotkey, *%A_LoopField%, KeyboardDummyLabel, %state% UseErrorLevel
   Return
KeyboardDummyLabel:
Return
}

;--------------------------------------------------------------------------------------------------------------

FadeOut(id)
{
IfWinExist, ahk_id %id%
{
	localvar=255
	Loop, 14
		{
			localvar:=localvar-18
			WinSet, Transparent, %localvar%, ahk_id %id%
			sleep 10
			
		}
	IfWinExist, ahk_id %id%
	Gui %id%:Destroy
	}	
}

;--------------------------------------------------------------------------------------------------------------

FadeIn(id)
{
	localvar=0
	loop, 17
	{
		WinSet, Transparent, %localvar% , ahk_id %id%
		sleep 10
		localvar:=localvar+15
	}	
}

;--------------------------------------------------------------------------------------------------------------

GetOSVersion() {
    Return ((r := DllCall("GetVersion") & 0xFFFF) & 0xFF) "." (r >> 8)
}