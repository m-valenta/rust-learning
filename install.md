## Setup wsl2
(mv_w_comp/mvPass01)

### Commandy kolem wls
``wsl --list`` list včech distribucí  
``wsl --unregister <Distro>`` odinstalace  
``wsl --install <Distro>`` instalace  
``wsl -e command``  
``wslconfig /setdefault <DistributionName>`` nastaví defaultní distro   

## Ubuntu/Linux 

### Commands -general
``cat [file_name] `` Write file content  
``mkdrir [dir_name]``  
``cp [file_name] [target_directory]``  
``cp -R [dir_name] [target_directory]``  
``rm file_name``  
``rmdir -r [dir_name]``  
``sudo`` root user  
``touch [file_name]`` zalozi soubor  
``ls -a`` listing všeho v aktualnim addresari (vcetne skrytych souboru)  
Win systém je namontovaný na ``/mnt/c/``  
Instalace toolů do sys.:  
``sudo apt update -y``  
``sudo apt install <tool name>``
Zavření terminálu ```exit``

### Commands - vi  
Start with ``vi [file_name]`` 
1. ``esc + i`` turn in-line editing mode (end with esc)
2. command can be entered after hitting ``:``  
``wq! + enter`` save and close   
``q! + enter`` close  

### Setup DNS on WSL 

```
sudo rm -r /etc/resolv.conf
sudo vi /etc/resolv.conf
```

_We need to add this line with DNS server from ipconfig_  
``nameserver xx.xx.xx.xx``
 
### Rust intallation
- Compiler toolchain ``sudo apt install build-essential``
- link [here](https://www.rust-lang.org/tools/install)
  - ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup rs | sh``
   - Kontrola pomocí ``ls ~/.cargo/bin``
   - Kontrola po restartu ``rustc --version``
- uninstall ``rustup self uninstall``
- update ``rustup update``

### Nový project (na ubuntu)
V cílovém adresáři (_po přípravě VSCode_) zavolat:    
``cargo init``  
``code .``

## VSCode

### Příprava
- __Na win__ 👉 je třeba do vs-code nainstalovat rozšíření: __WSL (Open any folder ...)__ 
- Aby to frčelo je třeba nainstalovat extensions (_ideálně až po otevření z ubuntu (objevi se install in wsl: ubuntu)_): 
  - rust-analyzer
  - CodeLLDB

### Práce (Ubuntu)
Lokace s projekty: ``cd '/mnt/c/Users/m.valenta_gmc/OneDrive - Quadient/Documents/Rust/Projects'``
Navigace do proj pomocí cd   
Spuštění vs-code ``code .``  
F5 (případně přidat target pro debugger)


__Přímá compilace__    
``cargo build`` build s cargem imo podobne npm install 
``cargo build --release`` build v release módu
``cargo run`` spusti program






