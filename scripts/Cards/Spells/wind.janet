(def cost 2)
(def on-play 
 @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  )])

(def description "He has seen better days")

(def display-image-asset-string "missing")
