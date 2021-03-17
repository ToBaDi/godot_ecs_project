extends Node


var scenario : RID
var viewport_rid : RID

onready var game := preload("res://aban/lib/game.gdns").new()


func _ready() -> void:
	viewport_rid = get_tree().root.get_viewport_rid()
	scenario = get_tree().root.world.scenario
	game.ready(scenario, viewport_rid)


func _process(delta : float) -> void:
	game.process(delta)


func _exit_tree() -> void:
	game.free()
