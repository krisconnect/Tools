# EventID for succesful or failed logins, LogonType for batch, RDP etc.
Get-WinEvent -LogName security -FilterXPath "*[System[EventID=4624] and EventData[Data[@Name='LogonType']='2']]"