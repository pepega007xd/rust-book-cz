## Instalace

Prvním krokem je nainstalovat Rust. Stáhnete jej pomocí `rustup`, nástoje k instalaci různých verzí Rustu a dalších pomocných nástrojů. Ke stažení budete potřebovat připojení k internetu.

> Poznámka: Pokud si z nějakého důvodu nepřejete použít `rustup`, navštivte [Další instalační metody][otherinstall].

Pomocí následujících kroků nainstalujete nejnovější stabilní verzi překladače Rustu. Záruky stability poskytované Rustem zajišťují, že všechny ukázky v této knize, které lze přeložit, bude možno přeložit i pomocí novějších verzí Rustu. Výstup překladače se může mírně lišit, neboť vývojáři často vylepšují chybové a varovné hlášky. Jinak řečeno, jakákoli novější stabilní verze Rustu by měla pro účely této knihy fungovat podle očekávání.

> ### Instalace v terminálu
>
> V této kapitole a ve zbytku knihy budeme používat terminálové příkazy. Každý terminálový příkaz začíná znakem `$`. Samotný znak `$` do terminálu nezadávejte, je to výzva terminálu k zadání dalšího příkazu a v knize slouží ke zvýraznění začátku příkazu. Řádky nezačínající znakem `$` obvykle obsahují výstup předchozího příkazu. Příkazy specifické pro PowerShell začínají znakem `>`.

### Instalace nástroje `rustup` na Linuxu a macOS

Pokud pracujete na Linuxu nebo v macOS, otevřete terminál a proveďte následující příkaz: 

```console
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Tento příkaz stáhne skript, který spustí instalaci nástroje `rustup`, pomocí kterého nainstalujete nejnovější stabilní verzi Rustu. Instalace může vyžadovat vaše heslo. Pokud instalace skočí úspěšně, uvidíte následující zprávu: 

```text
Rust is installed now. Great!
```

Také budete potřebovat *linker*, program používaný Rustem ke spojení přeložených kusů kódu do jednoho souboru. Pravděpodobně už máte linker nainstalovaný, pokud ale dostanete chyby spojené s linkerem, nainstalujte si překladač jazyka C, který bude linker obsahovat. C překladač budete dřív nebo později tak jako tak potřebovat, protože některé běžné balíčky Rustu závisí na C kódu, k jehož přeložení potřebujete C překladač. 

Na macOS nainstalujete C překladač následujícím příkazem: 

```console
$ xcode-select --install
```

Pokud používáte Linux, nainstalujte si GCC nebo Clang, a to podle návodu své distribuce. Například uživatelé Ubuntu budou potřebovat balíček `build-essential`.

### Instalace nástroje `rustup` ve Windows

V systému Windows navštivte [https://www.rust-lang.org/tools/install][install] a postupujte podle instalačních instrukcí. V průběhu instalace budete vyzváni k instalaci *MSVC build tools* pro Visual Studio 2013 nebo novější. Pro jejich získání budete potřebovat [Visual Studio 2022][visualstudio]. Až budete vybírat, které komponenty nainstalovat, vyberte:

- “Desktop Development with C++”,
- SDK pro Windows 10 nebo 11,
- jazykový balíček pro angličtinu, společně s libovolnými dalšími jazyky. 

Ve zbytku knihy jsou použity příkazy pro *cmd.exe* a PowerShell. Pokud budou v příkazech nějaké rozdíly, vysvětlíme vám, který použít.



### Řešení problémů

Pro kontrolu, že je Rust správně nainstalován, otevřete terminál a zadejte následující příkaz: 

```console
$ rustc --version
```

Měli byste vidět verzi, hash poslední změny a datum poslední změny stabilní verze Rustu, v následujícím formátu: 

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Pokud vidíte tyto informace, úspěšně jste nainstalovali Rust! Pokud ne, zkontrolujte, že je Rust zapsaný v systémové proměnné `%PATH%`.

Ve Windows CMD:

```console
> echo %PATH%
```

V PowerShellu:

```console
> echo $env:Path
```

V Linuxu a macOS:

```console
echo $PATH
```

Pokud je vše nastaveno správně a Rust stále nefunguje, můžete požádat o pomoc na několika místech. Nejjednodušší bude kanál #beginners na [Discord serveru Rustu][discord]. Můžete si tam psát s dalšími Rustaceans (anglická slovní hříčka - člen komunity Rustu), kteří vám pomohou. Mezi další zdroje informací patří [Uživatelské fórum][users] a [Stack Overflow][stackoverflow].

### Aktualizace a odinstalace

Jakmile je Rust nainstalován pomocí `rustup`, aktualizace na novou verzi je jednoduchá. Rust aktualizujete pomocí tohoto příkazu:

```console
$ rustup update
```

Pokud chcete Rust a `rustup` odinstalovat, zadejte následující příkaz:

```console
$ rustup self uninstall
```

### Místní dokumentace

Každá instalace Rustu obsahuje také kopii dokumentace, kterou můžete číst offline. Příkazem `rustup doc` ji otevřete ve svém prohlížeči.

Kdykoli si nebudete jistí, k čemu slouží nějaká funkce nebo typ standardní knihovny, podívejte se do dokumentace k její API.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust
