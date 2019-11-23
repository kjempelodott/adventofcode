class Player:

    def __init__(self, hp, damage, mana=0, spent=0, effects={}):
        self.hp = hp
        self.damage = damage
        self.mana = mana
        self.spent = spent
        self.effects = dict((k,v) for k,v in effects.items())
    
    def copy(self):
        return Player(self.hp, self.damage, self.mana, self.spent, self.effects)


class Spell(object):

    def __init__(self, name, cost, damage, armor, heal, recharge, duration=0):
        self.name = name
        self.cost = cost
        self.duration = duration
        self.damage = damage
        self.armor = armor
        self.heal = heal
        self.recharge = recharge

    def cast(self, player, boss):
        boss.hp -= self.damage
        player.armor += self.armor
        player.hp += self.heal
        player.mana += self.recharge


class Effect(Spell):

    def cast(self, player, boss):
        if player.effects.get(self.name, 0):
            super(Effect, self).cast(player, boss)
            player.effects[self.name] -= 1
            if not player.effects[self.name]:
                del player.effects[self.name]
        else:
            player.effects[self.name] = self.duration


spells = {'Magic Missile': Spell ('Magic Missile', 53, 4, 0, 0,   0),
          'Drain':         Spell ('Drain',         73, 2, 0, 2,   0),
          'Shield':        Effect('Shield',       113, 0, 7, 0,   0, 6),
          'Poison':        Effect('Poison',       173, 3, 0, 0,   0, 6),
          'Recharge':      Effect('Recharge',     229, 0, 0, 0, 101, 5)}


difficulty = 'medium'
def make_turn(player, boss, new_spell):

    def init():
        if difficulty == 'hard':
            player.hp -= 1
            if player.hp <= 0:
                return False

        player.armor = 0
        # Players turn
        for spell in player.effects.keys():
            spells[spell].cast(player, boss)

        if boss.hp <= 0:
            return True

    def cast_new():
        spell = spells[new_spell]
        player.mana -= spell.cost
        player.spent += spell.cost
        spell.cast(player, boss)
        if boss.hp <= 0:
            return True

    def boss_attack():
        # Boss turn
        for spell in player.effects.keys():
            spells[spell].cast(player, boss)

        if boss.hp <= 0:
            return True

        player.hp -= max(1, boss.damage - player.armor) + (difficulty == 'hard')
        if player.hp <= 0:
            return False

    casted = False
    while not casted:
        result = init()
        if result != None:
            return result

        if player.mana >= spells[new_spell].cost:
            if cast_new():
                return True
            casted = True

        result = boss_attack()
        if result != None:
            return result


mana = 500
player = Player(50, 0, mana)
boss = Player(55, 8)
min_mana = 2**15
min_comb = []


def i(_player, _boss, casted=[]):

    global min_mana, min_comb

    if _player.spent > min_mana:
        return

    for spell in spells:
        player = _player.copy()
        boss = _boss.copy()

        result = make_turn(player, boss, spell)
        if result == None:
            i(player, boss, casted + [spell])
        elif result:
            if player.spent < min_mana:
                min_mana = player.spent
                min_comb = casted + [spell]
            break

i(player, boss)
print min_mana, min_comb
