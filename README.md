# MultiToolRS (Rust verison)
MultiTool is a tool that can: Activate your OS with your KMS Server, clear temporary files, and more. 

Commands list:
----
  - -activate: Activate your OS using the KMS server 
  - -clear: Clear %TEMP% folder (Not ready)
  - -sfc: Check system files with SFC
  - -dism: Check system files with DISM
  - -help: Prints help menu
  - -about: Prints about menu


### "Under The Hood"
| Command        | Deploys           |
| -------------  |:------------------:|
| -activate      | 1. `slmgr /ipk {key}`, 2. `slmgr /skms {KmsServer}`, 3. `slmgr /ato`    |
| -dism          | `dism /online /cleanup-image /restorehealth`         |
| -sfc           | `sfc /scannow`        |
