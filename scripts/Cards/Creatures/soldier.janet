(def cost 1)
(def card-image "assets/image2.png")
(def movement-points 3)

(def movement(std/plus 1))
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

(def description "A soldier drafted from the finest of peasents")

(def display-image-asset-string "missing")
