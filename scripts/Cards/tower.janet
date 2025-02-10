(def cost 1)
(def card-image "../assets/image.jpg")

(def movement @[])

(def attack @[@[1 0] 
              @[0 1] 
              @[0 -1] 
              @[1 0] 
              @[1 1] 
              @[1 -1] 
              @[-1 -1] 
              @[-1 1]])

(def attack-strength 3)
(def defense 2)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] nil)
(defn on-turn-end [context] nil)
