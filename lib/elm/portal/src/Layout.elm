module Layout exposing (Model, view, viewCommands)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input
import Html exposing (Html)


type alias Model msg =
    { title : String
    , showWorkspace : Bool
    , content : Element msg
    , workspace : Element msg
    , actions : Element msg
    }


type alias Command msg =
    { onPress : msg, label : Element msg }


view : Model msg -> Html msg
view model =
    Element.layout [] <|
        column [ width fill, height fill, defaultSpacing ]
            [ viewHeader model
            , viewPage model
            , viewFooter model
            ]


viewCommands : List (Command msg) -> Element msg
viewCommands =
    \commands ->
        Element.column
            [ Border.widthEach { top = 0, right = 0, bottom = 0, left = 1 }
            , paddingEach { top = 4, right = 8, left = 10, bottom = 4 }
            , Border.color (Element.rgb255 145 145 145)
            , spacing 20
            ]
            (commands
                |> List.map
                    (\command ->
                        Element.Input.button
                            [ Font.size 14
                            , Font.family [ Font.typeface "system-ui" ]
                            ]
                            { onPress = Just command.onPress, label = command.label }
                    )
            )


headerStyle : Bool -> Length -> Length -> Color -> Color -> List (Attribute msg)
headerStyle extend len h bg bgAlt =
    (List.append [ width len
    , defaultPadding
    , height h
    ] (if not extend then  
            [ Background.color bg ]
        else  
           [ Background.gradient { 
                angle = 0.25 * pi, 
                steps = [ bg, bgAlt ]
            }]))

viewHeader : Model msg -> Element msg
viewHeader model =
        let
            width =
                if model.showWorkspace then
                    px 2048
                else 
                    fill
            height =
                px 80
            backgroundColor =
                rgb255 0x95 0xA5 0x8D 
            backgroundAltColor =
                rgb255 0x75 0xA5 0x5D 
        in
         row ( ( ( ( ( headerStyle
            <| model.showWorkspace )
            <| width )
            <| height )
            <| backgroundColor )
            <| backgroundAltColor )
            [ text model.title ]

viewPage : Model msg -> Element msg
viewPage model =
    let
        shrinkContent = 
            model.showWorkspace
    in
    row [ width fill, height fill, defaultSpacing, defaultPadding ]
        <| 
        if shrinkContent then 
            [ viewActions model
            , viewWorkspace model
            , viewContent model ]
        else 
            [ viewWorkspace model
            , viewContent model
            , viewActions model ]

viewFooter : Model msg -> Element msg
viewFooter model =
        let
            width =
                if model.showWorkspace then
                    px 2048
                else 
                    fill
            height =
                px 100
            backgroundColor =
                rgb255 0x95 0xA5 0x8D 
            backgroundAltColor =
                rgb255 0x75 0xA5 0x5D 
        in
         row ( ( ( ( ( headerStyle
            <| model.showWorkspace )
            <| width )
            <| height )
            <| backgroundColor )
            <| backgroundAltColor )
            [ text model.title ]

viewContent : Model msg -> Element msg
viewContent model =
    column
        [ width
            (fillPortion
                (if model.showWorkspace then
                    1

                 else
                    2
                )
            )
        , height fill
        , defaultPadding
        ]
        [ model.content ]


viewWorkspace : Model msg -> Element msg
viewWorkspace model =
    column
        [ width
            (fillPortion
                (if model.showWorkspace then
                    3

                 else
                    1
                )
            )
        , height fill
        , defaultPadding
        ]
        [ model.workspace ]


viewActions : Model msg -> Element msg
viewActions model =
    column
        [ width (fillPortion 1)
        , height fill
        , defaultPadding
        ]
        [ model.actions ]


defaultPadding : Attribute msg
defaultPadding =
    padding 20


defaultSpacing : Attribute msg
defaultSpacing =
    spacing 20
