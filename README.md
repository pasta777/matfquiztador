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

<p align="center">
  <img src="https://github.com/pasta777/matfquiztador/assets/146671691/00e895d0-159d-4ea5-b408-455667efcb34" alt="image">
</p>

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

<br>
<br>
<br>

The game ends:
a) if only one player remains
b) after a certain number of rounds (adjustable before the beginning of the game)

The winner of the game is the player who is the last one remaining, or the player who, after a certain number of rounds, has the most territories under their control.

If multiple players have the same number of territories, the winner is the one with the most successful defenses.

If, in that case, the winner remains undetermined, all players engage in an additional round with questions until a winner is decided.


# <p align="center" style="font-size:72px;">CONTROLS</p>

  LMB is used for all game controls and maneuvering menus. Use ESC to exit the game and close the application.


# <p align="center" style="font-size:72px;">AUTHORS</p>

Project for the course Programming Paradigms 3I of the Mathematical faculty, University of Belgrade.
Contributors:
* Stepan Ignjatović
* Mihajlo Trifunović
* Miloš Radovanović

# TO-DO LIST

* Come up with a plan, write down all the rules, and distribute tasks (Miloš) #DONE
* Implement all cities and connections between them (Mihajlo) #DONE
* Implement the stage of placing the main cities at the beginning (Mihajlo) #DONE
* Implement graphical representation of free, conquered, and capital territory (Stepan) #DONE
* Implement the stage of choosing free territories (Stepan) #DONE
* Implement the warfare stage (Stepan) #DONE
* Implement graphical representation of a player's attack on a territory, such as placing the player's flag on that territory as a declaration of attack (Stepan) #DONE
* Implement choosing the number of moves in the warfare phase at the beginning, i.e., the number of moves after which the game will default to ending (Stepan) #DONE
* Implement a system, whether external or internal, to keep track of the number of successful defenses of a player's territory (Stepan) #DONE
* Implement sudden death round in case of a tie by all parameters (Stepan) #CANCELED
* Implement a question database so that questions are not repeated during the game (Miloš) #DONE
* Tie questions to UI, randomly display answers to questions (Miloš) #DONE
* Tidy up the window and the overall look of the game (Stepan) #DONE
* Decide on adding a minimalist AI player or focusing on multiplayer and implement the chosen option (Stepan) #DONE
* Conduct code review, find better solutions for inefficient parts, and generally improve the project (Mihajlo) #DONE
* Test the game, find bugs, and fix them (All)

If you want to take on something from this list, write your name next to the item you want to work on. After completing that task, put #DONE next to it if you're satisfied with the processing.
