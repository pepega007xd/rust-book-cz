# Programování hádací hry

Práci v Rustu si nejdříve vyzkoušíme na praktickém projektu. Tato kapitola vám představí několik běžných konceptů Rustu, a to ve skutečném programu. Dozvíte se o klíčových slovech `let` a `match`, přidružených funkcích, používání beden a dalších konceptech. V následujících kapitolách se jimi budeme zabývat podrobněji, zde si jen projdeme základy.

Napíšeme známý program pro začátečníky: hádací hru. Zadání je následující: program vygeneruje náhodné celé číslo mezi 1 a 100. Poté vyzve hráče k zadání tipu. Poté program vypíše, zda je tip příliš nízký, nebo příliš vysoký. Pokud je tip správně, program vypíše blahopřání a ukončí se.

## Vytvoření nového projektu

Nový projekt vytvoříte tak, že přejdete do složky *projects* z kapitoly 1 a založíte nový projekt pomocí Carga, takto:

```console
$ cargo new guessing_game
$ cd guessing_game
```

První příkaz `cargo new` bere jako první argument název projektu (`guessing_game`). Druhý příkaz přejde do nově vytvořené složky projektu.

Prohlédněte si nově vytvořený soubor *Cargo.toml*:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run
-->

<span class="filename">Soubor: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Jak jste viděli v kapitole 1, `cargo new` vygeneruje "Hello, world!" program, viz soubor *src/main.rs*:

<span class="filename">Soubor: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Nyní tento program přeložíme a spustíme v jednom kroku příkazem `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Příkaz `cargo run` se hodí, když potřebujete rychle testovat sestavení vašeho projektu. Budeme to tak dělat i v tomto projektu, po každé větší změně projekt zkusíme sestavit.

Znovu si otevřete soubor *src/main.rs*, sem napíšeme všechen kód.

## Zpracování tipu

První část programu si vyžádá uživatelský vstup, zpracuje tento vstup a zkontroluje, zda je v očekávané formě. Zatím umožníme hráči jen zadat tip. Kód z ukázky 2-1 vložte do souboru *src/main.rs*.

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Ukázka 2-1: Kód, který přečte tip od uživatele a vypíše jej.</span>

Tento kód obsahuje spoustu informací, pojďme si ho tedy projít řádek po řádku. Abychom mohli získat uživatelský vstup a pak jej vypsalt jako výstup, musíme do rozsahu přidat vstupně-výstupní knihovnu `io`. Knihovna `io` je součástí standardní knihovny, známé jako `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Ve výchozím nastavení má Rust definovanou sadu objektů standardní knihovny, které uvede přidá do rozsahu každého programu. Tato sada se nazývá *úvod* a co všechno obsahuje si můžete přečíst v [dokumentaci standardní knihovny][prelude].

Pokud typ, který chcete použít, není v úvodu, musíte ho přidat do rozsahu přímo pomocí příkazu `use`. Knihovna `std::io` poskytuje řadu užitečných funkcionalit, včetně možnosti přijmout uživatelský vstup.

Jak jste viděli v první kapitole, funkce `main` je vstupním bodem programu:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

Syntax `fn` deklaruje novou funkci, závorky `()` značí, že nemá žádné parametry, a složená závorka (`{`) označuje začátek kódu kunkce.

Jak už jste také viděli v kapitole 1, `println!` je makro umožňující vypsat řetězec na obrazovku:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Tento kód vypisuje text, který vysvětluje hru a žádá uživatele o vstup.

### Ukládání hodnot do proměnných

Dále vytvoříme *proměnnou*, do které uložíme uživatelský vstup:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Teď už náš program vypadá zajímavěji. Na tomto řádku se toho děje hodně. Pomocí příkazu `let` vytváříme novou proměnnou, zde je další příklad:

```rust,ignore
let apples = 5;
```

Tento řádek vytvoří novou proměnnou s názvem `apples` a přiřadí jí hodnotu 5. V Rustu jsou bez dalších úprav proměnné neměnitelné, což znamená, že jakmile proměnné při vytvoření přiřadíme hodnotu, už ji později nemůžeme změnit. Tento koncept podrobně probereme v části ["Proměnné a měnitelnost"][variables-and-mutability]<!-- ignore --> v kapitole 3. Pokud chceme, aby proměnná byla měnitelná, přidáme před její název `mut`:

```rust,ignore
let apples = 5; // neměnitelná
let mut bananas = 5; // měnitelná
```

> Poznámka: Syntaxe `//` značí začátek komentáře, který pokračuje až do konce řádku. Rust obsah komentářů ignoruje. Komentáře probereme podrobněji v [kapitole 3][comments]<!-- ignore -->.

Vrátíme-li se k programu, nyní již víte, že `let mut guess` vytvoří měnitelnou proměnnou s názvem `guess`. Rovnítko (`=`) říká Rustu, že chceme proměnné přiřadit hodnotu. Napravo od rovnítka je hodnota přiřazovaná proměnné `guess`, která vznikla zavoláním `String::new` - funkce, která vrací novou instanci `String`. [`String`][string] je datový typ řetězce poskytovaný standardní knihovnou, je to rozšiřitelný text kódovaný pomocí UTF-8.

Syntaxe `::` v části `::new` značí, že `new` je *přidružená funkce* typu `String`. *Přidružená funkce* je funkce, která je implementovaná na nějakém typu, v tomto případě `String`. Tato funkce `new` vytvoří nový prázdný řetězec. Funkci `new` naleznete na mnoha typech, neboť je to běžný název pro funkci, která vytváří novou hodnotu typu.

Celý řádek `let mut guess = String::new();` vytvořil měnitelnou proměnnou, které přiřadil novou prázdnou instanci `String`. No teda!

### Získání uživatelského vstupu

Připomeňme si, že na prvním řádku jsme do programu přidali vstupně-výstupní funkcionality ze standardní knihovny pomocí `use std::io;`. Nyní zavoláme funkci `stdin` z modulu `io`, která nám umožní pracovat s uživatelským vstupem:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

I kdybychom na začátku programu neimportovali knihovnu `io` pomocí `use std::io`, mohli bychom tuto funkci stále používat voláním `std::io::stdin`. Funkce `stdin` vrací instanci [`std::io::Stdin`][iostdin]<!-- ignore -->, což je typ představující handle standardního vstupu vašeho terminálu.

Dále, řádek `.read_line(&mut guess)` zavolá metodu [`read_line`][read_line]<!--ignore --> na handlu standardního vstupu, čímž získá uživatelský vstup. `&mut guess` předáváme jako argument pro `read_line`, do tohoto řetězce se uloží uživatelský vstup. Celým úkolem `read_line` je vzít vše, co uživatel zadá do standardního vstupu a připojit to na konec řetězce (bez přepsání jeho obsahu), proto tento řetězec předáme jako argument. Řetězec musí být měnitelný, aby mohla metoda `read_line` změnit jeho obsah.

Znak `&` značí, že tento argument je *reference*, pomocí které může několik částí kódu přistupovat k jednomu kusu dat, aniž byste museli tato data vícekrát kopírovat do paměti. Reference jsou složitá funkcionalita a jednou z hlavních výhod Rustu je, že používání referencí je jednoduché a bezrizikové. K dokončení programu ale nepotřebujete znát takovéto podrobnosti, prozatím vám stačí vědět, že tak jako proměnné jsou i reference bez dalších úprav neměnitelné. Proto musíte místo `&guess` napsat `&mut guess` Referencemi se bude podrobněji zabývat kapitola 4.

### Zacházení s potenciální chybou pomocí typu `Result`

Pořád pracujeme s tímto řádkem kódu. Sice už jsme na třetím řádku textu, ale i ten stále součástí jediného logického řádku kódu. Jeho další částí je tato metoda:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Také bychom tento kód mohli napsat takto:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Chyba při čtení řádku");
```

Jeden dlouhý řádek je ale těžko čitelný, proto je lepší jej rozdělit. Při volání metody se syntaxí `.method_name()` je často rozumné přejít na nový řádek a přidat odsazení, pomůže vám to rozdělit dlouhé řádky. Nyní se podíváme na to, co tento řádek dělá.

Jak již bylo zmíněno, `read_line` vloží vživatelský vstup, do řetězce, který mu předáme, ale také vrátí hodnotu `Result`. [`Result`][result]<!--
ignore --> je druh [*výčtu*][enums]<!-- ignore -->, což je typ, který se může nacházet v jednom z více možných stavů. Každý možný stav nazýváme *variantou*.

Výčty se bude podrobněji zabývat kapitola 6. Účelem typů `Result` je předat informace o chybách.

Varianty výčtu `Result` jsou `Ok` a `Err`. Varianta `Ok` označuje, že operace byla úspěšná, a uvnitř je úspěšně vygenerovaná hodnota. Varianta `Err` značí, že se operace nezdařila a obsahuje informace o tom, jak nebo proč se tak stalo.

Hodnoty typu `Result` na sobě mají definované metody, stejně jako hodnoty jakéhokoli typu. Instance `Result` má metodu [`expect`][expect]<!-- ignore -->, kterou můžete zavolat. Pokud má tato instance `Result` variantu `Err`, metoda `expect` způsobí selhání programu a zobrazí zprávu, kterou jste jí předali jako argument. Když metoda `read_line` vrátí `Err`, je to nejspíš v důsledku chyby pocházející z operačního systému. Pokud má instance `Result` variantu `Ok`, `expect` že vezme návratovou hodnotu obsaženou uvnitř `Ok` a vrátí ji, abyste ji mohli použít. V tomto případě je to počet bajtů uživatelského vstupu.

Pokud `expect` nezavoláte, program sice půjde přeložit, ale dostanete varování:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust vás varuje, že jste nepoužili hodnotu `Result` vrácenou z `read_line`, což znamená, že váš program nepočítá s možností chyby.

Správným způsobem, jak předejít tomuto varování, je napsat skutečné zpracování chyby, ale v tomto případě bude stačit, když náš program selže jakmile dojde k chybě, proto nám stačí `expect`. O zpracování chyb se dozvíte více v [kapitole 9][recover]<!-- ignore -->.

### Vypisování hodnot pomocí zástupných symbolů v `println!`

Kromě zavírající složené závorky je tu už jen jeden řádek, který zbývá probrat:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Tento řádek vypíše řetězec, který nyní obsahuje uživatelský vstup. Složené závorky (`{}`) jsou zástupný symbol: představte si `{}` jako krabí klepítka, která drží hodnotu na místě. Pomocí složených závorek můžete vypsat více než jednu hodnotu: první složené závorky obsahují první hodnotu uvedenou za formátovacím řetězcem, druhé obsahují druhou hodnotu, atd. Vypsání více hodnot v jednom volání `println!` by vypadal takto:

```rust
let x = 5;
let y = 10;

println!("x = {} a y = {}", x, y);
```

Tento kód by vypsal `x = 5 a y = 10`.

### Testování první části

Pojďme otestovat první část našeho programu. Spustíme ji pomocí `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Uhádněte číslo.
Zadejte svůj tip:
6
Váš tip: 6
```

Nyní je první část našeho programu hotová: umíme získat vstup z klávesnice a poté ho vypsat.

## Generování tajného čísla

Dále musíme vygenerovat tajné číslo, které se uživatel pokusí uhodnout. Toto číslo by mělo být pokaždé jiné, aby byla zábava hrát více než jednou. Použijeme náhodné číslo mezi 1 a 100, aby hra nebyla příliš obtížná. Rust zatím ve své standardní knihovně nezahrnuje funkcionality pro náhodná čísla, proto tým Rustu poskytuje bednu [`rand`][randcrate], která tyto funkcionality dodá.

### Použití bedny na získání dalších funkcionalit

Pamatujte, že bedna je skupina souborů zdrojového kódu v Rustu. Projekt, na kterám pracujeme, je *binární bedna*, neboli spustitelný soubor. Bedna `rand` je *knihovna* - typ bedny, která obsahuje kód určený pro použití v jiných programech a kterou nelze spustit samostatně.

Schopnost spravovat externí bedny je jednou z hlavních výhod Carga. Než budeme moci napsat kód využívající `rand`, musíme upravit soubor *Cargo.toml* tak, aby zahrnoval bednu `rand` jako závislost. Otevřete tento soubor a přidejte následující řádek na jeho konec, pod hlavičku sekce `[dependencies]`, kterou vytvořilo Cargo. Ujistěte se, že zadáváte `rand` přesně tak, jako zde, se stejným číslem verze, jinak by ukázky kódu v tomto návodu nemusely fungovat.

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Soubor: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

V souboru *Cargo.toml* je součástí sekce vše, co následuje za její hlavičkou, a to až do začátku další sekce. V sekci `[dependencies]` sdělíte Cargu, na kterých externích bednách váš projekt závisí a které verze těchto beden potřebujete. V tomto případě požadujeme bednu `rand` sémantické verze `0.8.3`. Cargo rozumí [sémantickému verzování][semver]<!-- ignore --> (někdy nazývanému SemVer), standardu pro zápis verzí. Číslo `0.8.3` je ve skutečnosti zkrácenou formou zápisu `^0.8.3`, což zahrnuje jakoukoli verzi vyšší nebo rovnou 0.8.3, ale nižší než 0.9.0.

Cargo předpokládá, že tyto verze mají API kompatibilní s verzí `0.8.3`. Tato specifikace zajišťuje, že získáte nejnovější úroveň oprav, přičemž kód v této kapitole půjde stále přeložit. U verze `0.9.0` nebo vyšší není zaručeno, že bude mít stejné API, jaké používají následující příklady.

Nyní sestavíme náš projekt, zatím bez jakýchkoli změn v kódu, viz ukázka 2-2:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Ukázka 2-2: Výstup příkazu `cargo build` po přidání bedny `rand` jako závislosti</span>

Možná uvidíte jiná čísla verzí (ale všechny budou kompatibilní s naším kódem díky SemVer) a jiné řádky (v závislosti na operačním systému). Pořadí řádků se taky může lišit.

Když přidáme externí závislost, Cargo načte nejnovější verze všeho, co daná závislost potřebuje, z *registru*. Registr je kopie dat z [Crates.io][cratesio]. Crates.io je stránka, na které lidé v ekosystému Rustu zveřejňují své open source projekty, aby je ostatní mohli používat.

Po aktualizaci registru Cargo zkontroluje sekci `[dependencies]` a stáhne všechny uvedené bedny, které ještě nejsou stažené. V tomto případě, i když jsme uvedli jako závislost pouze `rand`, Cargo stáhlo také všechny bedny, na kterých `rand` závisí. Po stažení beden je Rust přeloží a následně přeloží i náš projekt s pomocí dostupných závislostí.

Pokud nyní spustíte `cargo build` znovu, aniž byste provedli jakékoli změny, nedostanete žádný výstup kromě řádku `Finished`. Cargo ví, že již stáhlo a přeložilo všechny závislosti, a že jste na nich v souboru *Cargo.toml* od té doby nic nezměnili. Cargo také ví, že jste nic nezměnili ve svém kódu, takže ani ten překládat nebude. A když nemá co dělat, jednoduše se ukončí.

Když otevřete soubor *src/main.rs*, provedete jednoduchou změnu a poté jej uložíte a sestavíte projekt, uvidíte na výstupu pouze dva řádky:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Na těchto řádcích vidíte, že Cargo aktualizuje sestavení pouze vaší drobnou změnou v souboru *src/main.rs*. Závislosti se nezměnily, takže Cargo může znovu využít vše, co už je stažené a přeložené.

#### Zajištění opakovatelných sestavení pomocí souboru *Cargo.lock*

Cargo má mechanismus, díky kterému můžete pokaždé sestavit přesně ten samý kód: dokud je sami nezměníte, Cargo bude používat pouze ty verze závislostí, které jste poprvé zadali. Řekněme například, že příští týden vyjde verze 0.8.4 bedny `rand`. Tato verze bude obsahovat důležitou opravu chyby, ale také regresi, kvůli které váš kód nepůjde přeložit. Z tohoto důvodu Cargo při prvním spuštění `cargo build` vytvoří soubor *Cargo.lock*, který nyní máme ve složce *guessing_game*.

Když poprvé sestavujete svůj projekt, Cargo načte všechny verze závislostí yhovující kritériím, a zapíše je do souboru *Cargo.lock*. Když budete později sestavovat svůj projekt znovu, Cargo uvidí, že projekt obsahuje soubor *Cargo.lock*, a použije v něm zapsané verze místo hledání nových. Díky tomu můžete zopakovat stejné sestavení. Jinými slovy, váš projekt zůstane díky souboru *Cargo.lock* na verzi `0.8.3`, dokud jej ručně neupgradujete. Vzhledem k důležitosti souboru *Cargo.lock* pro reprodukovatelná sestavení je tento soubor často zahrnut ve systému správy verzí společně se zbytkem kódu vašeho projektu.

#### Aktualizace beden pro získání nových verzí

Když *chcete* bednu aktualizovat, Cargo poskytuje příkaz `update`, který bude ignorovat soubor *Cargo.lock* a načte nejnovější verze beden, které vyhovují specifikacím v *Cargo.toml*. Tyto verze pak zapíše do souboru *Cargo.lock*. Cargo bude ve výchozím nastavení hledat pouze verze vyšší než `0.8.3` a nižší než `0.9.0`. Kdyby vyšly dvě nové verze bedny `rand`, `0.8.4` a `0.9.0`, po spuštění `cargo update` byste viděli následující:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cargo ignoruje verzi `0.9.0`. Nyní byste si také mohli všimnout změny v souboru *Cargo.lock*, a to, že aktuálně používaná verze bedny `rand` je `0.8.4`. Pokud chete přejít na verzi `0.9.0` nebo jakoukoli z řady `0.9.x`, musíte upravit soubor *Cargo.toml* následujícím způsobem:

```toml
[dependencies]
rand = "0.9.0"
```

Až příště spustíte `cargo build`, Cargo aktualizuje registr dostupných beden a stáhne novou verzi bedny `rand` odpovídající vašemu požadavku. 

O [Cargu][doccargo]<!-- ignore --> a jeho [ekosystému][doccratesio]<!-- ignore --> se dozvíte více v kapitole 14, zatím vám ale bude stačit, co už víte. Cargo usnadňuje používání knihoven, proto jsou uživatelé Rustu schopní psát menší projekty používající více balíčků.

### Generování náhodného čísla

Nyní použijeme `rand` na vygenerování náhodného čísla. Upravte soubor *src/main.rs* podle ukázky 2-3:

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Ukázka 2-3: Přidání kódu, který vygeneruje náhodné číslo</span>

Nejprve přidáme řádek `use rand::Rng`. Trait `Rng` definuje metody, které generátory náhodných čísel implementují. Tento trait musíme přidat do rozsahu, abychom tyto metody mohli používat. Traitům se bude podrobně věnovat kapitola 10.

Dále přidáme dva řádky uprostřed. V prvním řádku zavoláme funkci `rand::thread_rng` vracející generátor náhodných čísel, který budeme používat. Tento generátor existuje jen na aktuálním vlákně a je seedovaný operačním systémem. Poté zavoláme metodu `gen_range` definovenou na našem generátoru. Tato metoda je definována traitem `Rng`, který jsme přidali do rozsahu příkazem `use rand::Rng`. Metoda `gen_range` bere jako argument rozsah a vrací náhodné číslo v tomto rozsahu. Zde použitý druh rozsahu má tvar `začátek..=konec` a zahrnuje v sobě samotnou spodní i horní hranici. Pokud tedy chceme číslo mezi 1 a 100 včetně, musíme zadat `1..=100`.

> Poznámka: Většinou nebudete jen tak vědět, jaké traity použít a jaké metody a funkce konkrétní bedny volat, proto má každá bedna dokumentaci s pokyny k jejímu použití. Další elegantní funkcí Carga je, že spuštění příkazu `cargo doc --open` lokálně sestaví dokumentaci poskytovanou všemi závislostmi a otevře ji ve vašem prohlížeči. Například, pokud vás zajímají další funkcionality bedny `rand`, spusťte příkaz `cargo doc --open` a klikněte na `rand` na levém postranním panelu.

Druhý nový řádek vypíše naše tajné číslo. Bude se nám to hodit při psaní našeho programu za účelem testování, ale z konečné verze tento řádek smažeme. Nebyla by to moc dobrá hra, kdybychom uživateli vypsali odpověď hned na začátku.

Zkuste náš program párkrát spustit:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Uhádněte číslo.
Tajné číslo: 7
Zadejte svůj tip:
4
Váš tip: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Uhádněte číslo.
Tajné číslo: 83
Zadejte svůj tip:
5
Váš tip: 5
```

Měli byste dostat různá náhodná čísla mezi 1 a 100. Skvělá práce!

## Porovnání tipu a tajného čísla

Když už máme uživatelský vstup a náhodné číslo, můžeme je porovnat. Tento krok naleznete v ukázce 2-4. Všimněte si, že tento kód zatím nejde přeložit, jak bude vysvětleno později.

Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won’t compile quite yet, as we
will explain.

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Ukázka 2-4: Zpracování možných návratových hodnot z porovnání dvou čísel</span>

Nejprve přidáme další příkaz `use`, který přidá do rozsahu typ ze standarní knihovny zvaný `std::cmp::Ordering`. Typ `Ordering` je další výčet, jeho variantami jsou `Less`, `Greater` a `Equal`, neboli tři možné výsledky porovnání dvou hodnot.

Poté přidáme na konec pět nových řádků používajících typ `Ordering`. Metoda `cmp` porovnává dvě hodnoty a lze ji volat na čemkoli, co lze porovnávat. Jako argument bere referenci na cokoli, s čím chcete hodnotu porovnat - zde porovnáváme `guess` a `secret_number`. Metoda pak vrátí variantu výčtu `Ordering`, který jsme přidali do rozsahu příkazem `use`. Výraz [`match`][match]<!-- ignore --> rozhodne, co dělat dál, podle konkrétní varianty výčtu `Ordering`, která byla vrácena metodou `cmp`.

Výraz `match` tvoří *větve*. Každá větev se skládá ze *vzoru*, se kterým zadanou hodnotu porovnáváme, a kódu, který bude spuštěn, pokud hodnota odpovídá vzoru dané větve. Rust vezme hodnotu vloženou do výrazu `match` a postupně ji porovná se vzorem každé větve. Vzory a výrazy `match` jsou šikovné funkcionality Rustu, umožňující popsat různé situace, do kterých se váš kód může dostat, a zajistit jejich zpracování. Těmito funkcemi se budeme podrobněji zabývat v kapitolách 6 a 18.

Pojďme si projít příklad se zde použitým výrazem `match`. Řekněme, že uživatel tipnul 50 a náhodně vygenerované tajné číslo je v tomto případě 38. Když kód porovná 50 a 38, metoda `cmp` vrátí `Ordering::Greater`, protože 50 je větší než 38. Výraz `match` dostane hodnotu `Ordering::Greater` a začne procházet vzory všech větví. Podívá se na vzor první větve, `Ordering::Less`, a zjistí, že hodnota `Ordering::Greater` tomuto vzoru neodpovídá, proto ignoruje kód v této větvi a přesune se na další. Vzor další větve je `Ordering::Greater`, který *odpovídá* naší hodnotě `Ordering::Greater`. Přidružený kód této větve se spustí a vypíše `Too big!`. Vzhledem k tomu, že výraz `match` končí po první úspěšné shodě, s poslední větví už se hodnota porovnávat nebude.

Nicméně, kód v ukázce 2-4 zatím nepůjde přeložit. Zkusme to:

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Chyba v základu říká, že dochází k *neshodě typů*. Rust má silný, statický typový systém, zároveň ale umí typy odvozovat. Když jsme napsali `let mut guess = String::new()`, Rust odvodil, že `guess` je typu `String`, a nepožadoval po nás uvést typ ručně. Na druhé straně, `secret_number` je druh čísla. V Rustu existuje několik typů pro celá čísla, která mohou nabývat hodnot od 1 do 100: `i32`, 32bitové číslo; `u32`, 32bitové číslo bez znaménka; `i64`, 64bitové číslo; a další. Pokud není uvedeno jinak, Rust použije výchozí typ `i32`, což je také typ proměnné `secret_number`, alespoň dokud jinam v programu nepřidáte typování, kvůli kterým by Rust odvodil jiný číselný typ. Důvodem naší chyby je to, že Rust nemůže porovnat hodnoty typů řetězec a číslo.

Ve výsledku chceme převést `String`, který program přečetl na vstupu, na skutečný typ čísla, abychom ho poté mohli aritmeticky porovnat s tajným číslem. Toho docílíme přidáním následujícího řádku řádku do funkce `main`:

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Nový řádek:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Vytvoříme proměnnou s názvem `guess`. Ale moment, neexistuje již v programu proměnná s názvem `guess`? Existuje, ale Rust nám naštěstí umožňuje *zastínit* předchozí hodnotu `guess` novou hodnotou. Zastínění nám umožňuje znovu využít název `guess`, takž nemusíme vytvářet dvě samostatné proměnné, například `guess_str` a `guess`. Tomuto tématu se budeme podrobněji věnovat v kapitole 3, zatím vám bude stačit vědět, že tato technika se obvykle používá při změně typu proměnné.

Této nové proměnné přiřadíme hodnotu `guess.trim().parse()`. `guess` v tomto výrazu odkazuje na původní proměnnou `guess`, která obsahovala vstup ve formě řetězce. Metoda `trim` na zavolaná na instanci typu `String` odstraní všechny bílé znaky ze začátku a konce řetězce. To je potřeba, pokud chceme řetězec porovnat s `u32`, který může obsahovat pouze číselná data. Uživatel musí stisknout <span class="keystroke">enter</span>, aby funkce `read_line` vrátila jeho vstup, což do řetězce přidá znak nového řádku. Pokud například uživatel zadá <span class="keystroke">5</span> a stiskne <span class="keystroke">enter</span>, `guess` vypadá takto: `5\n`, kde `\n` představuje „nový řádek“. (Ve Windows stisknutí klávesy <span class="keystroke">enter</span> vloží znak CR a nový řádek, `\r\n`). Metoda `trim` odstraní znaky `\n` nebo `\r\n`, ponechávajíc pouze znak `5`.

Metoda [`parse` volaná na řetězci][parse]<!-- ignore --> převede daný řetězec na jiný typ. Zde ji používáme k převodu řetězce na číslo. Tentokrát musíme Rustu zadat přesný typ čísla, a to pomocí `let guess: u32`. Dvojtečka (`:`) za názvem proměnné `guess` říká Rustu, že bude následovat typová anotace proměnné. V Rustu existuje několik typů celých čísel; zde zobrazené `u32` je 32bitové číslo bez znaménka. `u32` je dobrá výchozí volba pro malé kladné číslo, a o dalších typech čísel se dozvíte v kapitole 3. Navíc, díky anotaci `u32` a porovnání s proměnnou `secret_number` Rust odvodí, že `secret_number` by mělo být také typu `u32`. Nyní už tedy budeme porovnávat dvě hodnoty stejného typu.

Metoda `parse` bude fungovat pouze u znaků, které lze smysluplně převést na čísla, a tím pádem může vrátit chybu. Pokud by například náš řetězec obsahoval `A👍%`, nebylo by možné jej převést na číslo. Protože může selhat, vrací metoda `parse` typ `Result`, podobně jako metoda `read_line` (viz ["Zacházení s potenciální chybou pomocí typu Result"](#zacházení-s-potenciální-chybou-pomocí-typu-result)). S touto instancí `Result` naložíme stejným způsobem a znovu použijeme metodu `expect`. Pokud metoda `parse` vrátí variantu `Err`, protože nedokázala z řetězce vytvořit číslo, zavoláním `expect` program selže a vypíše zprávu, kterou do něj zadáme. Pokud `parse` úspěšně převede řetězec na číslo, tak vrátí variantu `Ok` obsahující výsledek, a `expect` tím pádem vrátí číslo, které varianta `Ok` obsahuje.

Zkusme náš program spustit znovu.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Uhádněte číslo.
Tajné číslo: 58
Zadejte svůj tip:
  76
Váš tip: 76
Příliš velké číslo.
```

Skvěle! I když před náš tip přidáme mezery, program stále určí, že náš tip je 76. ZKuste program spustit několikrát a vyzkoušejte, jak se bude chovat s různými vstupy: zkuste správné číslo, příliš vysoká a příliš nízká čísla.

Většina hry už funguje, ale uživatel může hádat jenom jednou. Pojďme to spravit přidáním smyčky!

## Umožnění více tipů pomocí smyčky

Klíčové slovo `loop` vytvoří nekonečnou smyčku. Když přidáme smyčku, umožníme tím uživatelům hádat několikrát:

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Jak vidíte, přesunuli jsme vše od výzvy k zadání tipu do smyčky. Nezapomeňte odsadit řádky uvnitř smyčky o další čtyři mezery, a poté můžete program spustit znovu. Program bude nyní donekonečna žádat další tipy, což přináší nový problém - vypadá to, že by uživatel nemůže program ukončit.

Uživatel může vždy přerušit program pomocí klávesové zkratky <span class="keystroke">ctrl-c</span>. Existuje ještě další způsob, jak uniknout tomuto nenasytnému monstru, viz metoda `parse` v ["Porovnání tipu a tajného čísla"](#porovnání-tipu-a-tajného-čísla)<!-- ignore -->. Pokud uživatel zadá odpověď neobsahující číslo, program selže. Toho můžeme využít, abychom uživateli umožnili ukončit program:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Uhádněte číslo.
Tajné číslo: 59
Zadejte svůj tip:
45
Váš tip: 45
Příliš malé číslo.
Zadejte svůj tip:
60
Váš tip: 60
Příliš velké číslo.
Zadejte svůj tip:
59
Váš tip: 59
Vyhráli jste!
Zadejte svůj tip:
quit
thread 'main' panicked at 'Napište číslo.: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Napsání slova `quit` ukončí program, ale jak jste si všimli, stejně bude fungovat i vložení jakéhokoli jiného vstupu kromě čísel. To je přinejmenším nedokonalé; také bychom chtěli, aby se hra ukončila při uhádnutí správného čísla.

### Ukončení hry po správném tipu

Přidejme do hry kód, který ji ukončí po výhře pomocí příkazu `break`:

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Přidání příkazu `break` po `Vyhráli jste!` způsobí, že program opustí smyčku, jakmile uživatel správně uhodne tajné číslo. Ukončení smyčky také ukončí program, protože smyčka je poslední částí funkce `main`.

### Zpracování neplatného vstupu

Chování hry búžeme dále vylepšit tak, že namísto selhání programu při zadání neplatného tipu jej budeme ignorovat, aby uživatel mohl pokračovat ve hře. Toho docílíme změnou řádku, na kterém se `guess` převádí ze `String` na `u32`, jak je znázorněno v ukázce 2-5.

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Ukázka 2-5: Ignorujeme neplatný vstup a místo selhání programu požadujeme nový</span>

Přejdeme z volání `expect` na výraz `match`, pomocí kterého chybu zpracujeme místo selhání programu. Pamatujte, že `parse` vrací typ `Result` a že `Result` je výčet s variantami `Ok` a `Err`. Používáme zde výraz `match`, stejně jako jsme to udělali s výčtem `Ordering` u metody `cmp`.

Pokud `parse` úspěšně převede řetězec na číslo, vrátí hodnotu `Ok` obsahující dané číslo. Tato hodnota `Ok` bude odpovídat vzoru první větve a výraz `match` jednoduše vrátí hodnotu `num`, kterou metoda `parse` vložila do `Ok`. Toto číslo bude potom přiřazeno nové proměnné `guess`, kterou právě vytváříme.

Pokud `parse` *nedokáže* převést řetězec na číslo, vrátí hodnotu `Err` obsahující další informace o chybě. Hodnota `Err` nebude odpovídat vzoru `Ok(num)` v první větvi, ale bude odpovídat vzoru `Err(_)` v druhé větvi. Podtržítko `_` je všezahrnující hodnota - v tomto příkladě říkáme, že chceme zachytit všechny hodnoty `Err` bez ohledu na to, jaké informace obsahují. Program tedy provede kód druhé větve, `continue`, v důsledku čehož přejde k další iteraci smyčky `loop` a bude žádat o další tip. Takto ignorujeme všechny chyby, které by metoda `parse` mohla vrátit.

Nyní by mělo vše fungovat, jak má. Zkusme to:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`

Uhádněte číslo.
Tajné číslo: 61
Zadejte svůj tip:
10
Váš tip: 10
Příliš malé číslo.
Zadejte svůj tip:
99
Váš tip: 99
Příliš velké číslo.
Zadejte svůj tip:
foo
Zadejte svůj tip:
61
Váš tip: 61
Vyhráli jste!
```

Výborně! Jednou malou závěrečnou úpravou hru dokončíme. Připomeňme, že program stále vrací tajné číslo, což se nám hodilo při testování, ale do samotné hry to nechceme. Smažeme příkaz `println!`, který vypisuje tajné číslo. Ukázka 2-6 obsauje hotový kód.

<span class="filename">Soubor: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Ukázka 2-6: Hotový kód hádací hry</span>

## Shrnutí

Nyní jste úspěšně naprogramovali hádací hru. Gratulujeme!

Tento projekt byl praktickým představením spousty nových konceptů Rustu: `let`, `match`, funkcí, použití externích beden, atd. V několika dalších kapitolách se s těmito koncepty seznámíte podrobněji. Kapitola 3 se zabývá koncepty, které má Rust společné s většinou programovacích jazyků, jako jsou proměnné, datové typy a funkce, a ukazuje jejich použití v Rustu. Kapitola 4 řeší vlastnictví - funkcionalitu, která odlišuje Rust od ostatních jazyků. Kapitola 5 pojednává o strukturách a syntaxi metod, a kapitola 6 vysvětluje, jak fungují výčty.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#proměnné-a-měnitelnost
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
