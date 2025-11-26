(def zombie @{
'cost 2
'card-image "assets/image3.png"
'movement (std/plus 1)
'movement-points 1
'attack (array/join (std/cross 1) (std/plus 1))
'abilities @["dig"]
'attack-strength 2
'defense 1
'on-play @{
  :action {
    :type :apply-effect
    'effect :weakening
    'targeting ['area-around-caster 1]
    }
  'timing ['immidiate 'slow]
  'speed "slow"
  }


'description "He has seen better days"

'display-image-asset-string "missing"})
