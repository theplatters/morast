(def cost 2)
(def card-image "assets/image3.png")

(def movement (std/plus 1))
(def movement-points 1)

(def attack (array/join (std/cross 1) (std/plus 1)))

(def abilities @["dig"])

(def attack-strength 2)
(def defense 1)

(def on-draw @[])
(def on-play 
 @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  )])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin 
 @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  )])

(def on-turn-end @[])

