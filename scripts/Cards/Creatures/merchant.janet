(def cost 3)
(def movement-points 2)
(def card-image "assets/image.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def abilities @["fly" "dig"])
(def attack-strength 1)
(def defense 1)

(defn get-gold [game card-id] (if (std/is-owners-turn? game card-id) 
      (std/get-gold game 4 (std/owner game card-id))))

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[@{
  'action get-gold 
  'timing @['now]
  }])
(def on-turn-end @[@{
  'action get-gold
  'timing @['now]
  }])

(def description "Get 4 gold at the start and end of a turn")

(def display-image-asset-string "missing")
