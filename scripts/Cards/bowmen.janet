(def cost 1)
(def card-image "assets/image1.png")
(def movement (std/plus 1))
(def attack (array/join (std/plus 1) (std/plus 2)))


(def attack-strength 3)
(def defense 1)

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[])
(def on-turn-end @[])
