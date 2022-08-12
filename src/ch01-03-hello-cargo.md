## Ahoj, Cargo!

Cargo je sestavovací systém a správce balíčků pro Rust. Tento nástroj používá skoro každý, kdo pracuje s Rustem, protože Cargo za vás udělá spoustu úkonů. Stáhne knihovny, na kterých váš kód závisí (v Rustu jim říkáme *závislosti*), sestaví je, a poté sestaví váš kód.

Nejjednodušší programy, jako ten, co jsme doteď napsali, nemají žádné závislosti. Kdybychom náš "Ahoj, světe!" program sestavili pomocí Carga, použili bychom jenom tu část, která se stará o sestavení kódu. Jakmile ale budeme psát složitější programy, přidáme do projektu závislosti. Pokud takový projekt budeme spravovat pomocí Carga, velice si tím zjednodušíme práci.

Jelikož drtivá většina projektů v Rustu používá Cargo, zbytek této knihy bude přepokládat, že jej používáte také. Cargo je automaticky nainstalováno společně s Rustem, pokud jste použili oficiální instalátory, viz [sekce instalace][installation]<!-- ignore -->. Pokud jste Rust nainstalovali jiným způsobem, zkuste do terminálu zadat následující příkaz:

```console
$ cargo --version
```

Pokud vidíte číslo verze, máte Cargo nainstalované. Pokud vám terminál zobrazil chybu, např. `command not found`, podívejte se do dokumentace vaší instalační metody, jak Cargo nainstalovat zvlášť.

### Založení projektu s Cargem

Nyní vytvoříme nový projekt pomocí Carga a podíváme se, jak se liší od našeho původního projektu "Ahoj, světe!". Vraťte se do své složky *projects* (nebo kamkoli, kde ukládáte své projekty). Poté, nezávisle na vašem operačním systému, zadejte následující příkazy: 

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

První příkaz vytvoří novou složku a projekt nazvaný *hello_cargo*. Tím jsme náš projekt pojmenovali *hello_cargo*, a Cargo za nás vytvoří patřičné soubory ve složce stejného názvu.

Přejděte do složky *hello_cargo* a podívejte se na seznam souborů. Uvidíte že Cargo nám vygenerovalo dva soubory a jednu další složku: *Cargo.toml*, složku *src* a v ní soubor *main.rs*.

Také byl vytvořen nový Git repozitář společně se souborem *.gitignore*. Pokud spustíte `cargo new` ve složce, která už soubory Gitu obsahuje, Cargo je uvnitř projektu generovat nebude. Pokud je chcete i tak vygenerovat, použijte `cargo new --vcs=git`.

> Poznámka: Git je běžný verzovací systém. Pokud chcete použít jiný systém, nebo verzování úplně vypnout, použijte přepínač `--vcs`. Nalší možnosti zobrazíte pomocí `cargo new --help`.

Nyní si ve vašem editoru otevřete *Cargo.toml*. Obsah by měl být podobný ukázce 1-2.

<span class="filename">Soubor: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

<span class="caption">Ukázka 1-2: Obsah souboru *Cargo.toml* vygenerovaného pomocí `cargo new`</span>

Tento soubor je ve formátu [*TOML*](https://toml.io)<!-- ignore --> (*Tom’s Obvious,
Minimal Language*), který Cargo používá pro své konfigurační soubory.

První řádek, `[package]`, je hlavička sekce pro konfiguraci balíčku. Až budeme do tohoto souboru přidávat další informace, vytvoříme si další sekce.

Další tři řádky jsou konfigurační informace, které Cargo potřebuje k přeložení našeho programu: název, verzi a edici Rustu. Polem `edition` se budeme zabývat v [dodatku E][appendix-e]<!-- ignore -->.

Poslední řádek, `[dependencies]`, je hlavička sekce, kam budete zapisovat závislosti svého projektu. V Rustu nazýváme balíčky kódu *bedny* (crates). V tomto projektu nebudeme žádné bedny potřebovat, ale v kapitole 2 už ano, a použijeme k tomu tuto sekci.

Nyní otevřete soubor *src/main.rs*:

<span class="filename">Soubor: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo pro vás vygenerovat ukázkový "Ahoj, světe!" program, podobný tomu, který jsme napsali v ukázce 1-1. Rozdílem je to, že Cargo vložilo kód programu do složky *src*, a vytvořilo pro nás konfigurační soubor *Cargo.toml* ve složce projektu.

Cargo předpokládá, že vaše zdrojové soubory budou vždy ve složce *src*. Kořenová složka projektu je jen pro soubory README, licence, konfigurační soubory a další informace nesouvisející s vaším kódem. Cargo vám takto pomáhá organizovat vaše projekty - všechno má svoje místo.

Pokud jste vytvořili projekt, který nepoužívá Cargo, tak jako náš první "Ahoj, světe!" projekt, můžete jej změnit na projekt kompatibilní s Cargem tak, že všechen kód přemístíte do složky *src* a vytvoříte příslušný soubor *Cargo.toml*.



### Sestavení a spuštění projektu pomocí Carga

Nyní se podíváme na to, jak se liší sestavení a spuštění projektu, když máme k dispozici Cargo. Svůj projekt sestavíte tak, že ve složce projektu *hello_cargo* spustíte následující příkaz:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Takto vytvoříte binární soubor *hello_cargo(.exe)* ve složce *target/debug*. Vzhledem k tomu, že výchozí možností je sestavení pro ladění, Cargo umístí binární soubor do složky *debug*. Můžete jej spustit pomocí tohoto příkazu:

```console
$ ./target/debug/hello_cargo # nebo .\target\debug\hello_cargo.exe ve Windows
Hello, world!
```

Pokud vše fungovalo, ve svém terminálu byste měli vidět `Hello, world!`. Při prvním spuštění `cargo build` vytvoří Cargo nový soubor ve složce projektu - *Cargo.lock*. Do tohoto souboru Cargo zapisuje přesné verze závislostí vašeho projektu, ale vzhledem k tomu, že tento projekt žádné závislosti nemá, nyní tam toho moc nenajdete. *Cargo.lock* není třeba editovat ručně, Cargo jej bude měnit za vás.

Právě jsme sestavili projekt pomocí `cargo build` a spustili jej příkazem `./target/debug/hello_cargo`, ale můžeme si ušetřit práci příkazem `cargo run`, který projekt sestaví a hned spustí:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Příkaz `cargo run` je rychlejší než manuálně sestavovat aplikaci pomocí `cargo build` a potom ji spouštět zadáváním adresy binárního souboru, proto většina vývojářů používá `cargo run`.

Všimněte si, že tentokrát jsme neviděli zprávu oznamující, že Cargo překládá `hello_cargo`. Cargo si ověřilo, že jsme zdrojové soubory nijak nezměnili, proto jen spustilo už existující binární soubor. Kdybyste kód změnili, Cargo by jej nejdřív přeložilo, a pak spustilo novou verzi. V takovém případě byste viděli následující výstup:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo zároveň poskytuje příkaz `cargo check`, který zkontroluje, zda lze váš kód přeložit, ale nevytvoří binární soubor:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Proč byste nechtěli, aby Cargo váš kód skutečně přeložilo? `cargo check` je často o dost rychlejší, než `cargo build`, protože nedochází k samotnému generování binárního souboru. Pokud chcete svůj projekt průběžně kontrolovat, zda lze stále přeložit, používáním `cargo check` můžete uštřit čas. Spousta vývojářů používá `cargo check` průběžně při psaní programu, aby si byli jistí, že někde neudělali chybu. `cargo build` použijí až na konci, když potřebují svůj přeložený kód.

Připomeňme si, co jsme se zatím dozvěděli o Cargu:

* Nový projekt vytvoříme pomocí `cargo new`.
* Projekt můžeme sestavit pomocí `cargo build`.
* Můžeme jej sestavit a spustit jedním příkazem, a to `cargo run`.
* Také můžeme projekt sestavit bez generování binárního souboru příkazem `cargo check`.
* Cargo ukládá binární soubory do složky *target/debug* uvnitř našeho projektu.

Další výhodou Carga je, že na všech operačních systémech jsou jednotlivé příkazy shodné. Proto už odteď nebudeme rozlišovat kroky pro Linux, macOS a Windows.

### Sestavení pro vydání

Pokud je váš projekt připravený pro vydání, můžete použít příkaz `cargo build --release`, čímž povolíte optimalizace. Binární soubor naleznete ve složce *target/release* místo *target/debug*. Optimalizace zrychlí váš kód, ale samotný překlad bude trvat déle. Z tohoto důvodu rozlišujeme dva režimy: jeden pro samotný vývoj, kdy potřebujete překládat svůj kód často a rychle, a druhý pro překlad hotového programu, který budete dodávat svým uživatelům. Tento překlad nebude probíhat tak často, a výsledný kód bude tak rychlý, jak jen to je možné. Pokud chcete měřit rychlost svého programu, nezapomeňte překládat pomocí `cargo build --release` a měřit rychlost binárního souboru v *target/release*.

### Cargo jako konvence

V jednoduchých projektech vám Cargo nenabídne o moc více, než přímé používání překladače `rustc`. Cargo ale začne být užitečné, jakmile se váš kód rozroste. Jakmile bude mít váš program více zdrojových souborů, nebo bude potřebovat nějaké závislosti, sestavování pomocí Carga bude mnohem jednodušší. 

Projekt `hello_cargo` sice jednoduchý, i tak ale využívá většinu nástrojů, které budete při skutečné práci v Rustu používat. Pokud budete chtít pracovat na už existujícím projektu, většinou si vystačíte se stažením kódu pomocí Gitu, a sestavením pomocí Carga:

```console
$ git clone example.org/nejaky_projekt
$ cd nejaky_projekt
$ cargo build
```

Více informací o nástroji Cargo naleznete v [jeho dokumentaci].

[jeho dokumentaci]: https://doc.rust-lang.org/cargo/

## Shrnutí

Už teď máte dobrý základ pro svou cestu Rustem! V této kapitole jste je naučili, jak

* nainstalovat poslední stabilní verzi Rustu pomocí `rustup`,
* aktualizovat Rust na novou verzi,
* otevřít místní kopii dokumentace,
* napsat a spustit "Ahoj, světe!" program přímo pomocí `rustc`,
* vytvořit a spustit nový projekt konvenčně, pomocí Carga.

Teď bude nejlepší napsat o něco složitější program, abyste si zvykli na čtení a psaní Rustu. Proto v kapitole 2 naprogramujeme hádací hru. Pokud se chcete nejdřív naučit základní principy programování v Rustu, přeskočte na kapitolu 3, a pak se vraťte do druhé kapitoly.

[installation]: ch01-01-installation.html#instalace
[appendix-e]: appendix-05-editions.html
