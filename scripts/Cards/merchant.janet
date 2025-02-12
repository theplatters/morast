(def cost 3)
(def card-image "../assets/image.jpg")
(def movement @[@[1 0] 
              @[0 1] 
              @[0 -1] 
              @[1 0] 
              @[1 1] 
              @[1 -1] 
              @[-1 -1] 
              @[-1 1]])

(def attack @[@[1 0] 
              @[0 1] 
              @[0 -1] 
              @[1 0] 
              @[1 1] 
              @[1 -1] 
              @[-1 -1] 
              @[-1 1]])
(def attack-strength 3)
(def defense 3)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] (std/get-gold 3 (std/turn-player)))
(defn on-turn-end [context] nil)
