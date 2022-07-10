port module Main exposing (..)

import Browser
import Element exposing (..)
import Element.Input
import Editor exposing (viewMonacoEditor, viewMultilineEditor)
import Html exposing (..)
import Http
import Instructions
import Layout

type alias Model =
    { editor : Editor
    , instructions : String
    , viewFull : Bool
    }

type alias Editor =
    { text : String, language : String, saved : String }

type Msg
    = ResetText
    | Dispatch String
    | Save String
    | Instructions String
    | ViewFull
    | GotLab (Result Http.Error String)
    | Done

main : Program (Maybe String) Model Msg
main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- Init

init : (Maybe String) -> ( Model, Cmd Msg )
init maybelab =
    let
        default = 
            Model { text = "", language = "markdown", saved = "" } "" False
    in
    case maybelab  of
        Just lab -> 
            (default, getLab lab )
        Nothing -> 
            (default, getLab "" )

-- VIEW

view : Model -> Browser.Document Msg
view model =
    let
        viewFull =
            model.viewFull
    in
    { title = "Chiron lab portal"
    , body = (
        if viewFull then
        [ 
            Layout.view
            { title = ""
            , content = Instructions.viewFullPage model.instructions
            , right_detail = Element.text ""
            , left_detail = Element.text ""
            }
        ]
        else 
        [
            Layout.view
            { title = ""
            , content = Instructions.viewInstructions onNext ViewFull Done model.instructions
            , right_detail = Element.text ""
            , left_detail = Element.text ""
            }
        ])
    }

viewCodeEditor : { enableMonaco : Bool, model : Model } -> Element Msg
viewCodeEditor settings =
    let
        enableMonaco =
            settings.enableMonaco

        model =
            settings.model

        editor =
            { language = model.editor.language
            , text = model.editor.text
            }
    in
    if enableMonaco then
        Element.column [ width fill, height fill ]
            [ Element.Input.button []
                { onPress = Just (Dispatch "save")
                , label = Element.text "Render"
                }
            ,viewMonacoEditor editor
            ]

    else
        viewMultilineEditor Save editor

onNext : List String -> Maybe Msg
onNext remaining =
    if List.isEmpty remaining then
        Nothing
    else
        Just (Instructions (String.join "\n" remaining))

-- UPDATE

update : Msg -> Model -> ( Model, Cmd msg )
update msg model =
    let
        editor =
            model.editor
    in
    case msg of
        ResetText ->
            ( { model | editor = { editor | text = editor.saved } }, Cmd.none )

        Save content ->
            ( { model | editor = { editor | text = content, saved = content }, instructions = content }, Cmd.none )

        Dispatch cmd ->
            ( model, dispatchEditorCmd cmd )

        Instructions instructions ->
            ( { model | instructions = instructions }, Cmd.none )

        ViewFull -> 
            ( { model | editor = { editor | text = editor.saved }, instructions = model.editor.text, viewFull = True }, Cmd.none )

        Done ->
            ( { model | instructions = model.editor.text }, Cmd.none )
        
        GotLab result -> 
            case result of 
                Ok lab -> 
                    ({ model | editor = { editor | text = lab }, instructions = lab }, Cmd.none )
                Err _ -> 
                    (model, Cmd.none)


-- SUBSCRIPTIONS
-- If Monaco is enabled, this will allow us to pipe commands to the editor
port dispatchEditorCmd : String -> Cmd msg

-- This is called by monaco to pass the current value of it's editor
port saveContent : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions _ =
    saveContent Save

-- API

getLab : String -> Cmd Msg
getLab lab = 
    Http.get 
    { url = String.concat [ "/lab/", lab ]
    , expect = Http.expectString GotLab
    }