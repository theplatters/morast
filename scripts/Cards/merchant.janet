(def cost 3)
(def card-image "assets/image.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 1)
(def defense 1)

(def on-draw @[])
(def on-play @[])
(def on-discard @[])
(def on-ability @[])
(def on-turn-begin @[@{
  'action (fn [game] (if (= (std/owner game) (std/turn-player game)) 
      (std/get-gold game 4 (std/owner game))))
  'timing @['now]
  }])
(def on-turn-end @[@{
  'action (fn [game] (if (= (std/owner game) (std/turn-player game)) 
      (std/get-gold game 4 (std/owner game))))
  'timing @['now]
  }])
