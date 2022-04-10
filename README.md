1 ) initialize the game
2 ) player x plays
	-> check for state
2 ) player o plays
	-> check for state

game {
	baord_state ['x','o','n']
	game_state enum : player_x,player_o,draw,not_ended
	turn: player_x, player_o,a	
}

