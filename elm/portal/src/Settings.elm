module Settings exposing (..)

import Yaml.Decode exposing (..)

type alias Settings 
    = { id: String
      , display_name: String
      , description: String
      , depends_on: (List String)
      , tools: (List ToolSettings)
      }

type alias ToolSettings 
    = { name: String 
      , settings: (List String)
      }

    
