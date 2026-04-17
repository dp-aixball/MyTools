!include "MUI2.nsh"

; ==========================================================
; 基本配置
; ==========================================================
!define APP_NAME "Color Clicker"
!define EXE_NAME "color-clicker.exe"
!define VERSION "0.4.0" ; 这里可以后续通过参数传入

Name "${APP_NAME}"
OutFile "color-clicker-setup.exe"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
RequestExecutionLevel admin
!define MUI_ICON "../resources/icon.ico"
!define MUI_UNICON "../resources/icon.ico"

; ==========================================================
; 界面配置
; ==========================================================
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

; ==========================================================
; 安装逻辑
; ==========================================================
Section "Install"
    SetOutPath "$INSTDIR"
    
    ; 复制文件
    File "target\release\${EXE_NAME}"
    File "config.json"
    File "README.md"
    
    ; 创建卸载程序
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    ; 创建快捷方式
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${EXE_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
    CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${EXE_NAME}"
    
    ; 写入注册表以便在控制面板中显示卸载
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayName" "${APP_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "UninstallString" "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayIcon" "$INSTDIR\${EXE_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayVersion" "${VERSION}"
SectionEnd

; ==========================================================
; 卸载逻辑
; ==========================================================
Section "Uninstall"
    ; 删除文件
    Delete "$INSTDIR\${EXE_NAME}"
    Delete "$INSTDIR\config.json"
    Delete "$INSTDIR\README.md"
    Delete "$INSTDIR\Uninstall.exe"
    
    ; 删除快捷方式
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    Delete "$DESKTOP\${APP_NAME}.lnk"
    
    ; 清理注册表
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
    
    ; 删除目录
    RMDir "$INSTDIR"
SectionEnd
