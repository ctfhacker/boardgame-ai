class TicTacToeBoard(object):
    def start(self):
        """ Return initial board state at the beginning of a game """
        return [[0] * 3] * 3

    def current_player(self, state):
        """ Return current player based on the current game state """
        if state.count(0) % 2 == 1:
            return 1
        else:
            return 2
        

    def next_state(self, state, move):
        """ Apply the given move and then return the new game state """
        current_player = self.current_player(state)
        y, x = move
        state[y][x] = current_player
        return state

    def legal_moves(self, state_history):
        """ Return a list of legal moves based on the given state_history """
        moves = []
        for y in range(3):
            for x in range(3):
                if state[y][x] != 0:
                    continue
                moves.append((y, x))

        return moves


    def winner(self, state_history):
        """ Return a number based on if the game is current in a winning condition
            
            Player number of the winning player
            0 if the game is still ongoing
            -1 if the game is tied
        """
        wins = [
            # Columns
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            # Rows
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            # Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ]

        for win in wins:
            
            
