(def cost 1)
(def card-image "../assets/image.jpg")

(def movement @[])

(def attack (array/join (std/cross 1 ) (std/cross 2) (std/plus 2) (std/plus 1)))


(def attack-strength 3)
(def defense 2)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] nil)
(defn on-turn-end [context] nil)
