# <p align="center" style="font-size:200px;">MATFQUIZDATOR</p>

An application/game based on the popular mobile game Triviador, otherwise known as Conqueror, Conquistador, etc.

# <p align="center" style="font-size:72px;">GAME RULES</p>

The game is played with multiple players, where each player selects the location for their capital city on a given map at the beginning.

The order of selecting the capital city locations is random.

Capital cities cannot be adjacent to each other.

<p align="center">
  <img src="https://github.com/pasta777/matfquiztador/assets/146671691/18c44e78-f0d3-4095-aa3e-77c322c0ca53" alt="image">
</p>

After selecting the locations for the capital cities, the game proceeds to the phase of selecting territories, where each player, in the same order as in the selection of cities, chooses one free territory/city connected to their capital city or to any previously occupied territories/cities.

If a player does not have any free territories adjacent to their own territories, they can choose ANY other free territory on the map to occupy.




Once there are no more free territories, the game enters the phase of warfare.

The warfare phase is the final phase of the game, where players attempt to capture territories from each other.

Following the established order, players choose to attack a territory adjacent to one of theirs.

The attacking player and the defending player answer the same question.

If the attacking player answers correctly and the defending player answers incorrectly, the attack is successful, and the territory falls into the hands of the attacker.

If the situation is reversed, where the attacker answers incorrectly and the defender answers correctly, the attack fails, and the territory remains with the defender.

If both players answer correctly, a new question is asked to the players until one player makes a mistake. (SUBJECT TO CHANGE)

If the capital city is attacked, the attack must succeed three times for the capital city to be captured.

If the first attack on the capital city is successful, subsequent attacks happen immediately, and so on until the city is either defended or captured.

Upon capturing the capital city of a player, that player is eliminated from the game, and all their territories become the possessions of the conqueror of that capital city.

During their turn, a player can choose to secure/fortify their capital city by correctly answering a question. If successful, the city gains +1 life. If unsuccessful, the player's turn ends without any change.



The game ends:
a) if only one player remains
b) after a certain number of rounds (adjustable before the beginning of the game)

The winner of the game is the player who is the last one remaining, or the player who, after a certain number of rounds, has the most territories under their control.

If multiple players have the same number of territories, the winner is the one with the most successful defenses.

If, in that case, the winner remains undetermined, all players engage in an additional round with questions until a winner is decided.


# <p align="center" style="font-size:72px;">CONTROLS</p>

  LMB is used for everything.


# <p align="center" style="font-size:72px;">AUTHORS</p>

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
