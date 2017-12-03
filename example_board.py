class Board(object):
    def start(self):
        """ Return initial board state at the beginning of a game """
        pass

    def current_player(self):
        """ Return current player based on the current game state """
        pass

    def next_state(self, state, move):
        """ Apply the given move and then return the new game state """
        pass

    def legal_moves(self, state_history):
        """ Return a list of legal moves based on the given state_history """
        pass

    def winner(self, state_history):
        """ Return a number based on if the game is current in a winning condition
            
            Player number of the winning player
            0 if the game is still ongoing
            -1 if the game is tied
        """
        pass
