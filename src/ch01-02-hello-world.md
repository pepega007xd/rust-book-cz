## Ahoj, světe!

Když už máte Rust nainstalovaný, pojďme napsat váš první program v Rustu. Při začínání s novým jazykem je tradicí napsat jednoduchý program, který vypíše `Ahoj, světe!`, tak se do toho pustíme!

> Poznámka: Tato kniha předpokládá, že znáte základy používání terminálu. Rust nevyžaduje žádný konkrétní editor, takže pokud dáváte přednost vývojovému prostředí (IDE) před terminálem, klidně jej používejte. Mnoho vývojových prostředí má dnes určitou podporu Rustu, podrobnosti naleznete v dokumentaci daného IDE. Vývojáři Rustu se soustředí na podporu pomocí doplňku `rust-analyzer`, viz [dodatek D][devtools]<!-- ignore -->.

### Vytvoření složky projektu

Začneme vytvořením složky pro uložení vašho kódu. Rustu nezáleží na tom, kam budete ukládat svůj kód ukládat, ale pro přehlednost doporučujeme vytvořit pro všechny projekty v této knize složku *projects* umístěnou ve vaší domovské složce.

Následujícími příkazy vytvoříte složku *projects* a v ní složku pro náš "Ahoj, světe!" projekt.

Pro Linux, macOS a PowerShell:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Pro Windows CMD:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Psaní a spuštění programu v Rustu

Dále vytvořte nový soubor nazvaný *main.rs*. Zdrojové soubory Rustu vždy končí příponou *.rs*. Pokud název souboru tvoří víc než jedno slovo, dle konvence je oddělujte podtržítkem. Kupříkladu, *ahoj_svete.rs* bude lepší než *ahojsvete.rs*.

Nyní otevřete právě vytvořený soubor *main.rs* a vložte do něj kód z ukázky 1-1.

<span class="filename">Soubor: main.rs</span>

```rust
fn main() {
    println!("Ahoj, světe!");
}
```

<span class="caption">Ukázka 1-1: Program, který vypíše `Ahoj, světe!`</span>

Uložte soubor a vraťte se do terminálu ve složce *~/projects/hello_world*. Na Linuxu a macOS použijete k přeložení a spuštění programu následující příkazy:

```console
$ rustc main.rs
$ ./main
Ahoj, světe!
```
Ve Windows nahraďte `./main` příkazem `.\main.exe`:

```powershell
> rustc main.rs
> .\main.exe
Ahoj, světe!
```

Nezávisle na vašem operačním systému byste ve svém terminálu měli vidět řetězec `Ahoj, světe!`. Pokud vidíte něco jiného, požádejte o pomoc na odkazech v části [Řešení problémů][troubleshooting]<!-- ignore -->.

Pokud vidíte `Ahoj, světe!`, gratulujeme, právě jste napsali svůj prvni program v Rustu! Teď je z vás Rust programátor - vítejte!

### Anatomie Rust programu

Podívejme se podrobněji na náš "Ahoj, světe!" program. Toto je prnví díl skládačky: 

```rust
fn main() {

}
```

Těmito řádky definujete funkci `main`. Funkce `main` je speciální - je to první kód, který bude spuštěn v každém programu v Rustu. První řádek deklaruje funkci `main`, která nemá žádné parametry a nic nevrací. Kdyby nějaké parametry měla, byly by zapsány v závorkách `()`.

Samotný obsah funkce je ohraničený pomocí `{}`. V Rustu musíte obsah všech funkcí psát do složených závorek. Běžné je psát otevrítající složenou závorku na stejný řádek jako deklaraci funkce, s jednou oddělující mezerou.

> Poznámka: Pokud se chcete držet standarního stylu zápisu ve všech projektech, můžete použítnástroj `rustfmt`, který váš kód automaticky zformátuje, viz [dodatek D][devtools]<!-- ignore -->. Vývojáři Rustu přidali tento nástroj do standardní distribuce Rustu, tak jako `rustc`, takže byste jej měli mít mž nainstalovaný. 

Tělo funkce `main` obsahuje následující kód:

```rust
    println!("Ahoj, světe!");
```

Tento řádek odvádí všechnu práci našeho jednoduchého prgramu: vypíše text na obrazovku. Všimněte si čtyřech důležitých detailů: 

Zaprvé, v Rustu je zvykem odsazovat pomocí čtyř mezer, nikoli tabulátoru.

Zadruhé, `println!` zavolá v Rustu makro. Kdyby to bylo volání funkce, viděli bychom jen `println` (bez `!`). Markům v Rustu se budeme podrobněji věnovat v kapitole 19. Zatím potřebujete vědět jen to, že `!` znamená volání makra, nikoli normální funkce, a že makra se někdy řídí jinými pravidly než funkce.

Zatřetí vidíte řetězec `"Ahoj, světe!"`. Když tento řetězec předáme makru `println!` jako arguement, vypíšeme jej na obrazovku.

Začtvrté, ukončujeme řádek středníkem (`;`), což značí že tento výraz je u konce a začíná zde nový. Většina řádků v Rustu končí středníkem.

### Překlad a spuštění jsou dva kroky

Právě jste spustili nově vytvořený program, nyní si každý krok podrobněji rozebereme.

Předtím, než spustíte program v Rustu, musíte jej přeložit pomocí příkazu `rustc`, s názvem souboru jako argument:

```console
$ rustc main.rs
```

Pokud jste zvyklí psát C nebo C++, tento krok odpovídá použití `gcc` nebo `clang`. Po úspěšném překladu dostanete binární spustitelný soubor.

Na Linuxu, macOS a v Powershellu se můžete přesvědčit zadáním příkazu `ls`. Na Linuxu a macOS uvidíte dva soubory, ve Windows uvidíte tři soubory, v Powershellu i pomocí CMD:

```console
$ ls
main  main.rs
```

Ve Windows CMD zadejte následující:

```cmd
> dir /B %= přepínač /B zobrazí pouze názvy souborů =%
main.exe
main.pdb
main.rs
```

Uvidíte zdrojový kód s příponou *.rs*, spustitelný soubor (*main.exe* ve Windows, *main* na ostatních platformách), a ve Windows ještě soubor obsahující informace pro ladění s příponou *.pdb*. Nyní můžete spustit *main* nebo *main.exe*:

```console
$ ./main # nebo .\main.exe ve Windows
```

Pokud je *main.rs* váš "Ahoj, světe!" program, tento příkaz vypíše do terminálu `Ahoj, světe!`.

Pokud jste více obeznámení s dynamickými jazyky, jako např. Ruby, Python nebo JavaScript, nejspíš nejste zvyklí program překládat a spouštět ve dvou krocích. Rust je *překládaný jazyk*, což znamená, že můžete svůj kód přeložit a binární soubor dát ostatním. Ti pak budou schopní váš program spustit, aniž by měli nainstalovaný Rust. Pokud někomu dáte soubor *.rb*, *.py* nebo *.js*, každý kdo jej bude chtít spustit si bude muset nainstalovat interpreta Ruby, Pythonu nebo JavaScriptu. V těchto jazycích vám stačí jeden příkaz na překlad a spuštění programu. Vývoj jazyků je o kompromisech.

Překlad pouze pomocí `rustc` je dostačující pro jednoduché programy, ale jakmile začne váš projekt růst, budete potřebovat všchna nastavení někam napsat, aby byl váš kód snadněji přenosný. V další části vám představíme nástroj Cargo, se kterým budete psát skutečné programy v Rustu.

[troubleshooting]: ch01-01-installation.html#Řešení-problémů
[devtools]: appendix-04-useful-development-tools.md
