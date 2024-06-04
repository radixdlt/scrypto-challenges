export const sortEntities = (
  entities: Record<string, string>,
  iconUrlMap: Record<string, string>
) =>
  Object.entries(entities).reduce<{
    packages: string[]
    components: string[]
    tokens: { address: string; iconUrl: string }[]
  }>(
    (acc, [key, value]) => {
      const isPackage = value.startsWith('package')
      const isComponent = value.startsWith('component')
      const isResource = value.startsWith('resource')

      if (isPackage) return { ...acc, packages: [...acc.packages, value] }
      else if (isComponent)
        return { ...acc, components: [...acc.components, value] }
      else if (isResource) {
        const iconUrl = iconUrlMap[key]
        return {
          ...acc,
          tokens: [...acc.tokens, { address: value, iconUrl }],
        }
      }
      return acc
    },
    {
      packages: [],
      components: [],
      tokens: [],
    }
  )
