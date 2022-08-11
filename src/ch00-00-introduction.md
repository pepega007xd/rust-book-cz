# Úvod

Vítejte v *Programovacím jazyce Rust*, knize pro začátečníky v Rustu. S jazykem Rust můžete psát rychlejší a spolehlivějši software. Vysokoúrovňová ergonomika a nízkoúrovňový přístup jsou u většiny jazyků v konfliktu. Rust je výjimkou - vyvažuje možnosti které poskytuje a pohodlnost používání. Rust vám dává možnost přistupovat k nízkoúrovňovým detailům (např. správě paměti), aniž byste se dostali do potíží spojených s podobnými operacemi.

## Pro koho je Rust

Rust je ideálním jazykem pro spoustu různých skupin. Podívejme se na některé z těch nejdůježitějších.

### Týmy vývojářů

Rust se prosazuje jako produktivní nástroj pro spolupráci velkých týmů vývojářů s různými úrovněmi znalostí systémového programování. Nízkoúrovňový kód je náchylný k nenápadných chybám, které se ve většině jazyků dají odhalit jen důkladným testováním a podrobnou kontrolou kódu zkušenými vývojáři. V Rustu funguje překladač jako pojistka, odmítne přeložit kód obsahující tyto nenápadné chyby, včetně souběhových chyb. Když překladač spolupracuje s týmem, mohou se vývojáři věnovat logice programu místo ladění.

Rust také přináší moderní vývojářské nástroje do světa systémového programování:

* Cargo, vestavěný správce závislostí a sestavovací nástroj zjednodušuje přidání, překládání a správu závislostí skrze celý ekosystém Rustu.
* Rustfmt zajišťuje jednotný styl kódu pro všechny vývojáře.
* Rust Language Server zajišťuje zobrazování chybových zpráv a automatické doplňování kódu ve vývojových prostředích (IDE)

Používáním těchto a dalších nástrojů v ekosystému Rustu mohou vývojáři zvýšit svou produktivitu při psaní systémového kódu.

### Studenti

Rust je ideální pro studenty a pro ostatní, kteří se zajímají o systémové programování. Pomocí Rustu se spousta lidí učí např. vývoj operačních systémů. Komunita Rustu je přívětivá a ochotná zodpovídat otázky studentů. Pomocí projektů jako je tato kniha se vývojáři Rustu se snaží zpřístupnit systémové programování většímu množství lidí, zvlášť nováčků.

### Společnosti

Stovky společností, velkých i malých, používají Rust ve svých produktech, mezi které spadají např. terminálové aplikace, webové služby, DevOps nástroje, vestavěné systémy, analýza a překódování obrazu a zvuku, kryptoměny, bioinformatika, vyhledávací nástroje, IoT aplikace, strojové učení, a dokonce i důležité části prohlížeče Firefox.

### Vývojáři otevřeného softwaru

Rust je jazykem pro všchny, kteří přispívat do samotného jazyka Rust, komunitních projektů, vývojářských nástrojů a knihoven. Budeme vděční za vaše příspěvky do jazyka Rust.

### Lidé, kteří si cení rychlosti a stability

Rust je pro lidi, kteří touží po rychlosti a stabilitě. Rychlostí myslíme rychlost samotných programů napsaných v Rustu a rychlost, jakou je budete psát. Překladač Rustu obsahuje kontrolory zajišťující stabilitu při přidávání nových funkcí a změnách v kódu. V tom se Rust liší od starších jazyků, které tyto kontrolory neobsahují, v důsledku čehož se vývojáři často obávají změn ve starých systémech. Rust se snaži dosáhnout abstrakcí bez přidání náročnosti programu a vysokoúrovňových funkcionalit překladatelných do kódu rychlého jako ručně napsaný nízkoúrovňový kód. Rust usiluje o bezrizikový, ale zároveň rychlý kód. 

Rust se dá využít na spoustě dalších míst, tyto zmíněné jsou jen ty nejvýznamnější. Poskytnutím bezrizikovosti *a* produktivity, rychlosti *a* ergonomičnosti se Rust snaží odstranit kompromisy, ke kterým se programátoři na dekády uchýlili. Zkuste to a uvidíte, zda se vám Rust zamlouvá.

## Pro koho je tato kniha

Tato kniha předpokládá, že už máte nějakou zkušenost s psaním kódu, ale nepředpokládá znalost žádného konkrétního jazyka. Snažili jsme se napsat tuto knihu tak, aby byla pokud možno přístupná všem, nezávisle na vašich konkrétních zkušenostech. Netrávíme zde moc času rozebíráním, *co* je to programování, nebo jak o něm přemýšlet. Pokud jste ještě nikdy nic neprogramovali, zvažte spíš nějakou knihu určenou přímo začátečníkům v programování.

## Jak číst tuto knihu

Tato kniha v zásadě předpokládá, že ji čtete od začátku do konce. Pozdější kapitoly staví na konceptech představených v předchozích kapitolách. V začátcích se často nebudeme věnovat detailům, ke kterým se vrátíme v pozdějších částech.

V této knize najdete dva druhy kapitol: konceptuální kapitoly a projektové kapitoly. V konceptuálních kapitolách poznáte různé části Rustu. V projektových kapitolách budeme společně sestavovat menší programy, čímž si zopakujete, co jste se předtím naučili. Kapitoly 2, 12 a 20 jsou projektové, zbytek je konceptuální.

V kapitole 1 se dozvíte, jak nainstalovat Rust, jak napsat "Ahoj světe!" program a jak používat Cargo, správce balíčků a sestavovací nástroj pro Rust. Kapitola 2 obsahuje praktický úvod do Rustu, podíváme se zde na základní koncepty, které podrobně prostudujeme v dalších kapitolách. Pokud si chcete Rust zatím jen prohlédnout, začněte právě ve druhé kapitole. Možná budete dokonce chtít přeskočit třetí kapitolu, v níž se seznámíte s částmi Rustu, které jsou podobné ostatním jazykům. V kapitole 4 už vás bude čekat systém vlastnictví. Nicméně, pokud jste opravdu pečliví a chcete nejdříve rozumět každému detailu, než jej aplikujete, přeskočte kapitolu 2, a vraťte se k ní až po přečtení třetí kapitoly. Ve druhé kapitole uplatníte nově nabyté znalosti v malém projektu.

Kapitola 5 se zabývá strukturami a metodami, šestá kapitola výčty `enum`, výrazy s `match` a řídící strukturu `if let`. Naučíte se, jak pomocí struktur a výčtů vytvářet v Rustu vlastní datové typy.

V sedmé kapitole se budete zabývat moduly v Rustu a pravidly přístupu, abyte mohli efektivně organizovat svůj kód a jeho rozhraní (API). Kapitola 8 rozebírá běžné datové struktury kolekcí zahrnuté ve standardní knihovně, např. vektory, řetězce a hashovací tabulky. V kapitole 9 zjistíte, jak v Rustu nakládat s chybami.

Kapitola 10 rozebírá generické typy, traity a životnosti, pomocí kterých můžete psát kód obecný pro více datových typů. Kapitola 11 se zabývá testováním, které je potřeba k zaručení logické správnosti vašeho programu i přes všechny záruky, které Rust poskytuje. V kapitole 12 napíšeme zjednodušenou implementaci nástroje `grep`, určeného k hledání textu v souborech. K tomu budeme potřebovat obsah většiny předchozích kapitol.

V kapitole 13 projdeme uzávěry a iterátory - součásti Rustu pocházející z funkcionálního programování. V kapitole 14 se budeme podrobněji věnovat Cargu, projdeme také zásady sdílení knihoven s ostatními. Kapitola 15 se zabývá chytrými ukazateli zahrnutými ve standardní knihovně a traity, díky kterým fungují.

v kapitole 16 se budeme zabývat různými způsoby souběžnosti a tím, jak Rust zajišťuje bezpečnost vícevláknových programů. V sedmnácté kapitole zjistíte, jak uplatnit možnosti Rustu s běžnými technikami objektově orientovaného programování.

Kapitola 18 rozšiřuje téma vzorů a porovnávání vzorů - efektivních metod vyjadřování logiky v Rustu. Kapitola 19 obsahuje všehochuť pokročilejších témat, včetně rizikového Rustu, maker, dalších informací o životnostech, traitech, datových typech, funkcích a uzávěrech.

V kapitole 20 napíšeme projekt, ve kterém implementujeme nízkoúrovňový, vícevláknový webový server.

Na konci najdete několik dodatků s dalšími informacemi o Rustu psaných spíše referenčním způsobem. Dodatek A obsahuje klíčová slova Rustu, dodatek B operátory a další symboly, dodatek C odvoditelné traity poskytovacé standardní knihovnou. Dodatek D prochází vývojářské nástroje, dodatek E vysvětluje, co jsou to edice Rustu. V dodatku F naleznete další překlady této knihy a v dodatku G se dozvíte, jak probíhá vývoj Rustu, a co jsou to denní sestavení Rustu.

Neexistuje žádný šatný způsob, jak číst tuto knihu - pokud chcete přeskočit pár kapitol, s chutí do toho. Vždy se můžete vrátit zpátky, pokud něčemu nebudete rozumět. Je to na vás.

<span id="ferris"></span>

Důležitou součástí učení je pochopit, jak správně číst chyby překladače - pomohou vám opravit váš kód. Proto zde uvádíme spousty ukázek kódu, který nepůjde přeložit, spolu s ukázkami chybových hlášek překladače. Pamatujte, že ne všechny ukázky v této knize lze přeložit! Přečtěte si vždy přilehlý text, zda má daná ukázka být přeložitelná. Ferris vám pomůže odlišit, který kód nemá fungovat:

| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris s otazníkem"/>                     | Tento kód nelze přeložit!                        |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris se zvednutýma rukama"/>                      | Tento kód selže!                                 |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris s jedním zvednutým klepetem"/> | Tento kód se nechová tak, jak by měl.            |

Ve většině případů vás postupně dovedeme k tomu, jak nefunkční kód opravit do správné podoby.

## Zdrojový kód

Zdrojové soubory, ze kterých je tato kniha vygenerována, naleznete na [GitHubu][book].

[book]: https://github.com/pepega007xd/rust-book-cz/tree/main/src
