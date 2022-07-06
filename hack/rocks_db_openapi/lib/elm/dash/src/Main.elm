module Main exposing (..)

import Browser
import Http exposing (..)
import Json.Decode as Decode exposing (Decoder, field, string)
import Html exposing (Html)
import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Input as Input
import List exposing (any)
import Dict

-- App

main =
    Browser.element 
        { init = init
        , view = view
        , subscriptions = subscriptions
        , update = update
        }

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

init: () -> (Model, Cmd Msg)
init _ = 
    ( 
    { query = ""
    , users = [ Dict.singleton "test" (User "test" "test-id" "test-user-name") ]
    }, 
    getObjects )
    
type Msg
    = Search Model | GotUsers (Result Http.Error (List (Dict.Dict String User)))
    

type alias User = 
    { connected_registry_id: String
    , user_id: String
    , user_name: String
    }
type UserMap = 
    Dict String (Decoder User)

userDecoder : Decoder User
userDecoder =
    Decode.map3 User 
        (field "connected_registry_id" string)
        (field "user_id" string)
        (field "user_name" string)

userListDecoder : Decoder (List (Dict.Dict String User))
userListDecoder = 
    Decode.list userMapDecoder

userMapDecoder : Decoder (Dict.Dict String User)
userMapDecoder =
    Decode.map (Dict.map mapUsers) (Decode.dict userDecoder)

mapUsers : String -> User -> User
mapUsers _ { connected_registry_id, user_id, user_name } = 
    User connected_registry_id user_id user_name

getObjects : Cmd Msg
getObjects = 
    Http.get 
    { url = "/api/config/users"
    , expect = Http.expectJson GotUsers userListDecoder
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model = 
    case Debug.log "msg" msg of
        Search new -> 
            (new, Cmd.none)
        GotUsers result -> 
            case result of 
                Ok objs -> 
                    ({ model | users = objs }, Cmd.none)
                Err _ -> 
                    (model, Cmd.none)

type alias Model = 
    { query: String
    , users: List (Dict.Dict String User)
    }

-- Styles
dropShadow : Attr decorative msg
dropShadow = 
    Border.shadow 
        { blur = 2
        , color = (rgba 0 0 0 0.2)
        , offset = (-1.0, 1.0)
        , size = 0.5 }

cardStyle : List (Attr () msg)
cardStyle = 
       [ Background.color (rgb255 255 255 255)
       , Border.rounded 8
       , dropShadow
       , width (fill |> minimum 600) 
       , height (px 240)
       , padding 16
       ]
       
cardContentLayoutStyle : List (Attribute msg)
cardContentLayoutStyle = 
     [ spaceEvenly
     , padding 10
     , height fill
     , width fill 
     ]

cardLayoutStyle : List (Attribute msg)
cardLayoutStyle =
    [ width (fillPortion 3)
    , spacing 16
    ]

contextLayoutStyle : List (Attribute msg)
contextLayoutStyle =
    [ width (fillPortion 1)
    , height (px 600)
    , alignTop
    , dropShadow
    ]

searchBoxStyle : List (Attribute msg)
searchBoxStyle = 
    [ width (fill |> minimum 600) 
    , Border.rounded 8
    , padding 8
    , dropShadow
    ]

-- View

view : Model -> Html Msg
view model = 
    Element.layout []
     <| column [width fill]
          [ header
          , pageLayout model
          ]

header : Element msg
header = 
    row 
        [ width fill
        , height (px 80)
        , Background.color (rgb255 0xA5 0xA5 0x8D)
        ] []

pageLayout : Model -> Element Msg
pageLayout model =
    row 
        [ width (px 880)
        , centerX
        , padding 24
        , spacing 24
        ] 
        [ cardLayout model
        , contextLayout 
        ]

cardLayout : Model -> Element Msg
cardLayout model =
    column [height fill, spacing 16] 
    [ searchBox model
    , column cardLayoutStyle (cards model)
    ]

card : String -> Element msg
card content =
    el cardStyle
       ( row cardContentLayoutStyle (cardContent content) )

cardContent: String -> List (Element msg)
cardContent content = 
    [ (text content)
    ]

contextLayout : Element msg
contextLayout =
    column contextLayoutStyle [(text "content")]

-- Search Box / Card List

filter: String -> List (String) -> List (String)
filter query elements =
    List.filter (\content -> String.contains query content) elements 

searchBox : Model -> Element Msg
searchBox model = 
    Input.text searchBoxStyle
    { placeholder = Just (Input.placeholder [] (text "Repository name or tag..."))
    , label = Input.labelHidden "Search"
    , text = model.query
    , onChange = \new -> Search { model | query = new }
    }  

cards : Model -> List (Element msg)
cards model = 
    List.map card (filter model.query (List.concat (List.map (\l -> (Dict.keys l)) model.users)))




