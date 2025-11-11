(def cost 1)
(def card-image "assets/image2.png")

(def movement (array/join (std/plus 1) (std/plus 2)))
(def attack (std/plus 1))

(def abilities @["fly" "dig"])
(def attack-strength 3)
(def defense 3)

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[])
(def on-turn-end @[])
