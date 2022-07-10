module Layout exposing (Model, view, viewCommands)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Font
import Element.Input
import Html exposing (Html)

type alias Model msg =
    { title : String
    , shrinkContent: Bool
    , content : Element msg
    , left_detail : Element msg
    , right_detail: Element msg
    }

type alias Command msg = 
    { onPress : msg, label: Element msg }

view : Model msg -> Html msg
view model =
    Element.layout [] <|
        column [ width fill, height fill, defaultSpacing ]
            [ viewHeader model
            , viewPage model
            , viewFooter model
            ]

viewCommands : (List (Command msg)) -> Element msg 
viewCommands =
    (\commands ->
                Element.column 
            [ Border.widthEach { top = 0, right = 0, bottom = 0, left = 1 }
            , paddingEach { top = 4, right = 8, left = 10, bottom = 4}
            , Border.color (Element.rgb255 145 145 145)
            ] (
                commands |> List.map
                    (\command -> 
                    Element.Input.button 
                    [ Element.Font.size 14
                    , Element.Font.family [  Font.typeface "system-ui" ]
                    ] { onPress = Just command.onPress, label = command.label }
                    )
            )
    )

viewHeader : Model msg -> Element msg
viewHeader model =
    row
        [ width fill
        , defaultPadding
        , height (px 80)
        , Background.color (rgb255 0x95 0xA5 0x8D)
        ]
        [ text model.title ]


viewPage : Model msg -> Element msg
viewPage model =
    row [ width fill, height fill, defaultSpacing, defaultPadding ]
        [ viewDetail model
        , viewContent model
        , viewGutter model
        ]
viewFooter : Model msg -> Element msg
viewFooter _ =
    row
        [ width fill
        , defaultPadding
        , height (px 100)
        , Background.color (rgb255 0x95 0xA5 0x8D)
        ]
        []

viewContent : Model msg -> Element msg
viewContent model =
    column
        [ width (fillPortion (
            if model.shrinkContent then
                1
            else
                2
        ))
        , height fill
        , defaultPadding
        ]
        [ model.content ]

viewDetail : Model msg -> Element msg
viewDetail model =
    column
        [ width (fillPortion (
            if model.shrinkContent then
                3
            else
                1
        ))
        , height fill
        , defaultPadding
        ]
        [ model.left_detail ]

viewGutter : Model msg -> Element msg
viewGutter model =
    column
        [ width (fillPortion 1)
        , height fill
        , defaultPadding
        ]
        [ model.right_detail ]


defaultPadding : Attribute msg
defaultPadding =
    padding 20


defaultSpacing : Attribute msg
defaultSpacing =
    spacing 20
