// @generated
// This file was automatically generated and should not be edited.

import ApolloAPI

protocol ClawMasterAPI_SelectionSet: ApolloAPI.SelectionSet & ApolloAPI.RootSelectionSet
where Schema == ClawMasterAPI.SchemaMetadata {}

protocol ClawMasterAPI_InlineFragment: ApolloAPI.SelectionSet & ApolloAPI.InlineFragment
where Schema == ClawMasterAPI.SchemaMetadata {}

protocol ClawMasterAPI_MutableSelectionSet: ApolloAPI.MutableRootSelectionSet
where Schema == ClawMasterAPI.SchemaMetadata {}

protocol ClawMasterAPI_MutableInlineFragment: ApolloAPI.MutableSelectionSet & ApolloAPI.InlineFragment
where Schema == ClawMasterAPI.SchemaMetadata {}

extension ClawMasterAPI {
  typealias SelectionSet = ClawMasterAPI_SelectionSet

  typealias InlineFragment = ClawMasterAPI_InlineFragment

  typealias MutableSelectionSet = ClawMasterAPI_MutableSelectionSet

  typealias MutableInlineFragment = ClawMasterAPI_MutableInlineFragment

  enum SchemaMetadata: ApolloAPI.SchemaMetadata {
    static let configuration: any ApolloAPI.SchemaConfiguration.Type = SchemaConfiguration.self

    static func objectType(forTypename typename: String) -> ApolloAPI.Object? {
      switch typename {
      case "AgentMutation": return ClawMasterAPI.Objects.AgentMutation
      case "BoolResult": return ClawMasterAPI.Objects.BoolResult
      case "ModelInfo": return ClawMasterAPI.Objects.ModelInfo
      case "ModelQuery": return ClawMasterAPI.Objects.ModelQuery
      case "MutationRoot": return ClawMasterAPI.Objects.MutationRoot
      case "QueryRoot": return ClawMasterAPI.Objects.QueryRoot
      case "SessionEntry": return ClawMasterAPI.Objects.SessionEntry
      case "SessionQuery": return ClawMasterAPI.Objects.SessionQuery
      case "StatusInfo": return ClawMasterAPI.Objects.StatusInfo
      default: return nil
      }
    }
  }

  enum Objects {}
  enum Interfaces {}
  enum Unions {}

}