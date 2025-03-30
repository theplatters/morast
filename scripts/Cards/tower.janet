(def cost 1)
(def card-image "assets/image3.png")

(def movement @[])

(def attack (array/join (std/cross 1 ) (std/cross 2) (std/plus 2) (std/plus 1)))

(def abilities @["fly" "dig"])

(def attack-strength 3)
(def defense 2)

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[])
(def on-turn-end @[])
