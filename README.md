# <p align="center" style="font-size:85px;">MATFQUIZDATOR</p>

An application/game based on the popular mobile game Triviador, otherwise known as Conqueror, Conquistador, etc.
<div style="text-align:center;">
    ![image](https://github.com/pasta777/matfquiztador/assets/146671691/1cc52bd4-2149-4dcd-ba2e-ac80383e36b2)
</div>

# GAME RULES

Igra se igra u više igrača, gde svaki igrač na početku nad datom mapom bira gde želi da postavi svoj glavni grad.

Redosled odabira mesta za glavni grad je nasumičan.

Glavni gradovi ne smeju biti jedan pored drugoga.

Nakon odabira mesta za glavni grad prelazi se u fazu odabira teritorija gde svaki igrač istim redom kao što su birani gradovi bira po jednu slobodnu teritoriju/grad koju će zauzeti a koja je povezana sa glavnim gradom ili nekom prethodno zauzetom teritorijom/gradom.

Ukoliko igrač nema nijednu slobodnu teritoriju u dodiru sa svojim teritorijama, može da odabere BILO KOJU drugu slobodnu teritoriju na mapi i da je preuzme.

Nakon što nema više slobodnih teritorija, prelazi se u fazu ratovanja.

Faza ratovanja je poslednja faza igre, gde igrači međusobno pokušavaju da otmu jedni drugima teritoriju.

Prethodno ustanovljenim redosledom igrači biraju da napadnu neku teritoriju koja se dodiruje sa njihovom.

Igrač koji napada i igrač koji se brani odgovaraju na isto pitanje.

Ukoliko igrač koji napada odgovori tačno a igrač koji brani teritoriju pogreši, napad je uspešan i teritorija prelazi u ruke napadača.

Ukoliko se desi obratno, gde napadač pogreši a odbrana odgovori tačno, napad je neuspešan i teritorija ostaje u rukama odbrane.

Ukoliko oba igrača odgovore tačno, igračima se postavlja novo pitanje sve dok jedan igrač ne pogreši. (SUBJECT TO CHANGE)

Ukoliko se napada glavni grad, odbrana mora da zakaže tri puta da bi glavni grad bio osvojen.

Ukoliko je prvi napad na glavni grad bio uspešan, odvija se odmah i drugi i tako dok se ili grad ne odbrani ili grad ne osvoji.

Pri osvajanju glavnog grada nekog igrača taj igrač je eliminisan iz igre i sve njegove teritorije prelaze u posed osvajača tog glavnog grada.

Igrač tokom svog poteza umesto da napadne može da bira da osigura/popravi svoj glavni grad tako što će odgovoriti tačno na pitanje, u slučaju uspeha grad dobija +1 život, u slučaju neuspeha potez tog igrača se završava bez promene.


Igra se završava:
a) ako je preostao samo jedan igrač
b) posle određenog broja poteza (podesivo na početku partije)

Pobednik igre je igrač koji je jedini preostao ili igrač koji posle određenog broja poteza ima najviše teritorija pod svojom kontrolom. 

Ukoliko više igrača ima isti broj teritorija, pobednik je onaj sa najviše uspešnih odbrana.

Ukoliko bi i u tom slučaju pobednik bio neodređen, svi igrači igraju dodatnu rundu sa pitanjima, dok se ne odluči pobednik.


<p align="center">#AUTHORS</p>

Project for the course Programming Paradigms 3I of the Mathematical faculty, University of Belgrade.
Contributors:
* Stepan Ignjatović
* Mihajlo Trifunović
* Miloš Radovanović

# TO-DO LIST

* Smisliti plan, ispisati sva pravila i raspodeliti taskove (Miloš) DONE
* Implementirati sve gradove i grane povezanosti između njih (Mihajlo) DONE
* Implementirati etapu postavljanja glavnih gradova na početku (Mihajlo) DONE
* Implementirati grafički prikaz slobodnih, osvojenih kao i teritorija na kojima je glavni grad. (Stepan) DONE
* Implementirati etapu biranja slobodnih teritorija (Stepan) DONE
* Implementirati etapu ratovanja (Stepan) DONE
* Implementirati grafički prikaz napada nekog igrača na neku teritoriju, recimo postavljanja barjaka tog igrača na tu teritoriju kao deklaraciju napada. (Stepan) DONE
* Implementirati odabir broja poteza u fazi ratovanja na početku, odnosno broj poteza posle kojih će igra po difoltu biti gotova (Stepan) DONE
* Implementirati sistem, bilo eksterni ili interni, kojim će se pamtiti broj uspešnih odbrana teritorije nekog igrača. (Stepan) DONE
* Implementirati sudden death rundu u slučaju nerešene partije po svim parametrima (Stepan) CANCELED
* Implementirati bazu sa pitanjima, tako da se pitanja u bilo kom trenutku ne ponavljaju u toku igre. (Miloš) DONE
* Vezati pitanja za UI, nasumično prikazivati odgovore na pitanja. (Miloš) DONE
* Srediti prozor, i sam izgled igre. (Stepan) DONE
* Odlučiti se dodavanje minimalističkog AI igrača ili fokusiranje na igru za više igrača i implementirati odabrano. (Stepan) DONE
* Uraditi reviziju koda, pronaći bolja rešenja na neefikasne stvari i jednostavno poboljšati projekat (Mihajlo) ACCOMPLISHED
* Testirati igru, pronaći bagove i popraviti ih (Svi) IN PROGRESS

Ukoliko želite da preuzmete nešto sa ove liste, upišite vaše ime npr. (Stojan) pored stavke kojom želite vi da se bavite. Nakon finaliziranje te stavke stavite #DONE pored stavke ukoliko ste zadovoljni obradom.
