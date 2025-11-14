(def cost 3)
(def movement-points 2)
(def card-image "assets/image.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def abilities @["fly" "dig"])
(def attack-strength 1)
(def defense 1)

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[@{
  'action (fn [game card-id] (if (std/is-owners-turn? game card-id) 
      (std/get-gold game 4 (std/owner game card-id))))
  'timing @['now]
  }])
(def on-turn-end @[@{
  'action (fn [game card-id] (if (std/is-owners-turn? game card-id) 
      (std/get-gold game 4 (std/owner game card-id))))
  'timing @['now]
  }])
