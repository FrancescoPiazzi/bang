struct Action{
    source: ActionSource,
    target: &Character,
    action_type: ActionType
}

enum ActionSource{
    PILE,
    CHARACTER(&Character)
}

enum ActionType{
    SHOOT(amount),
    HEAL(amount),
    GIVE_ARROWS(amount)
}