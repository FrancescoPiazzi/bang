struct Action{
    source: ActionSource,
    target: &PlayableCharacter,
    action_type: ActionType
}

enum ActionSource{
    PILE,
    CHARACTER(&PlayableCharacter)
}

enum ActionType{
    SHOOT(amount),
    HEAL(amount),
    GIVE_ARROWS(amount)
}