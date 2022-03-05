import { Box, Flex } from "@chakra-ui/react"
import * as React from "react"
import Header from "components/Header"
import Head from "next/head"

export const Page: React.FC<{
    title?: string
    children?: JSX.Element | JSX.Element[] | React.ReactNode
}> = ({ title, children }) => {
    return <>
    <Head>
        <title>{title}</title>
    </Head>
        <Header/>
        <Box minH="100vh" h="100%">
            <Flex justifyContent="center" w="100%">
                <Box w="100%">{children}</Box>
            </Flex>
        </Box>
    </>
}