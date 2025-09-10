@echo off
REM TaskMaster Wrapper for Windows Command Prompt
REM Enforces research requirements before task operations
REM Usage: tm [command] [args]

powershell -ExecutionPolicy Bypass -File "%~dp0task-master-wrapper.ps1" %*