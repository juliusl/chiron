port module Main exposing (..)

import Browser
import Element exposing (..)
import Element.Input
import Html exposing (..)
import Http
import Instructions
import Layout


main : Program (Maybe String) Model Msg
main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- MODEL


type alias Model =
    { editor : Editor
    , instructions : String
    }


type alias Editor =
    { text : String, language : String, saved : String }


type Msg
    = ResetText
    | Dispatch String
    | Save String
    | Instructions String
    | GotLab (Result Http.Error String)
    | Done


init : (Maybe String) -> ( Model, Cmd Msg )
init maybelab =
    let
        default = 
            Model { text = "", language = "markdown", saved = "" } ""
    in
    case maybelab  of
        Just lab -> 
            (default, getLab lab )
        Nothing -> 
            (default, getLab "" )

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

-- VIEW

view : Model -> Browser.Document Msg
view model =
    { title = "Chiron lab portal"
    , body =
        [ 
            -- Layout.view
            -- { title = "Editor"
            -- , content = viewCodeEditor { enableMonaco = True, model = model }
            -- , detail = Instructions.viewInstructions onNext Done model.instructions
            -- },
            Layout.view
            { title = ""
            , content = Instructions.viewInstructionsFullPage model.instructions
            , right_detail = Element.text ""
            , left_detail = Element.text ""
            }
        ]
    }

onNext : List String -> Maybe Msg
onNext remaining =
    if List.isEmpty remaining then
        Nothing
    else
        Just (Instructions (String.join "\n" remaining))

sampleLab : String
sampleLab =
    """
# Lab - Setup a dev-box 
```yaml
id: "devbox-setup-2"
display_name: "Devbox Setup"
description: "Learn how to setup your own devbox"
depends_on:
- kind
tools:
- az_cli:
  - az vm create
```
## Generate an ssh key
```yaml
requires:
- id_rsa_pub:
tools:
- bash:
  - ssh-keygen
```
"""

getLab : String -> Cmd Msg
getLab lab = 
    Http.get 
    { url = String.concat [ "/lab/", lab ]
    , expect = Http.expectString GotLab
    }